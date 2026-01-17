//! An asynchronous event system for the CLI IDE.
//!
//! The [`Event<T>`] type implements a true publish-subscribe (broadcast) pattern
//! where **all** subscribers receive **every** emitted value of type `T`. This is
//! distinct from load-balancing where each message goes to only one consumer.
//!
//! It supports functional transformations such as `map`, `filter`, and `debounce`
//! to build event pipelines, similar to VS Code's event API.
//!
//! # Broadcast Semantics
//!
//! When you call [`Event::emit`], the value is cloned and sent to every active
//! subscriber. If a subscriber's channel is disconnected (receiver dropped),
//! it is automatically removed from the subscriber list.
//!
//! ```ignore
//! let event: Event<i32> = Event::new();
//! let sub1 = event.subscribe();
//! let sub2 = event.subscribe();
//!
//! event.emit(42);
//! // Both sub1 and sub2 receive 42
//! ```

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam::channel::{unbounded, Receiver, Sender};

/// An event stream producing values of type `T` with broadcast semantics.
///
/// Each call to [`subscribe`](Event::subscribe) creates a new independent channel.
/// When [`emit`](Event::emit) is called, the value is broadcast to **all** subscribers.
pub struct Event<T: Clone + Send + 'static> {
    subscribers: Arc<Mutex<Vec<Sender<T>>>>,
}

impl<T: Clone + Send + 'static> Clone for Event<T> {
    fn clone(&self) -> Self {
        Self {
            subscribers: Arc::clone(&self.subscribers),
        }
    }
}

impl<T: Clone + Send + 'static> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + 'static> Event<T> {
    /// Create a new `Event` with no subscribers.
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Broadcast a value to **all** current subscribers.
    ///
    /// The value is cloned for each subscriber. Subscribers whose channels have
    /// been disconnected (receiver dropped) are automatically removed.
    pub fn emit(&self, value: T) {
        let mut subs = self.subscribers.lock().expect("subscriber lock poisoned");
        // Retain only subscribers that successfully receive the message
        subs.retain(|sender| sender.send(value.clone()).is_ok());
    }

    /// Create a new subscription to this event.
    ///
    /// Returns a [`Receiver`] that will receive all values emitted after this
    /// call. Each subscriber gets its own independent channel, ensuring true
    /// broadcast semantics where every subscriber receives every event.
    pub fn subscribe(&self) -> Receiver<T> {
        let (sender, receiver) = unbounded();
        let mut subs = self.subscribers.lock().expect("subscriber lock poisoned");
        subs.push(sender);
        receiver
    }

    /// Apply a mapping function to each value in the stream, returning a new event.
    ///
    /// The returned event broadcasts transformed values to all of its subscribers.
    pub fn map<U, F>(self, f: F) -> Event<U>
    where
        U: Clone + Send + 'static,
        F: Fn(T) -> U + Send + Sync + 'static,
    {
        let downstream = Event::<U>::new();
        let downstream_clone = downstream.clone();
        let upstream_receiver = self.subscribe();

        thread::spawn(move || {
            for val in upstream_receiver.iter() {
                let mapped = f(val);
                downstream_clone.emit(mapped);
            }
        });

        downstream
    }

    /// Filter events based on a predicate.
    ///
    /// The returned event broadcasts only values that satisfy the predicate.
    pub fn filter<F>(self, predicate: F) -> Event<T>
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        let downstream = Event::<T>::new();
        let downstream_clone = downstream.clone();
        let upstream_receiver = self.subscribe();

        thread::spawn(move || {
            for val in upstream_receiver.iter() {
                if predicate(&val) {
                    downstream_clone.emit(val);
                }
            }
        });

        downstream
    }

    /// Emit values at most once every `duration` (throttle/debounce).
    ///
    /// The first value is always emitted. Subsequent values are only emitted
    /// if at least `duration` has passed since the last emission.
    pub fn debounce(self, duration: Duration) -> Event<T> {
        let downstream = Event::<T>::new();
        let downstream_clone = downstream.clone();
        let upstream_receiver = self.subscribe();

        thread::spawn(move || {
            let mut last_emit: Option<Instant> = None;
            for val in upstream_receiver.iter() {
                let now = Instant::now();
                let should_send = match last_emit {
                    Some(prev) => now.duration_since(prev) >= duration,
                    None => true,
                };
                if should_send {
                    downstream_clone.emit(val);
                    last_emit = Some(now);
                }
            }
        });

        downstream
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_event_emit_and_subscribe() {
        let event: Event<i32> = Event::new();
        let receiver = event.subscribe();

        event.emit(42);
        event.emit(100);

        let val1 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        let val2 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();

        assert_eq!(val1, 42);
        assert_eq!(val2, 100);
    }

    #[test]
    fn test_event_default() {
        let event: Event<String> = Event::default();
        let receiver = event.subscribe();

        event.emit("hello".to_string());
        let val = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val, "hello");
    }

    #[test]
    fn test_event_broadcast_to_multiple_subscribers() {
        // This test verifies TRUE broadcast semantics:
        // ALL subscribers receive ALL events (not load-balanced)
        let event: Event<i32> = Event::new();

        let sub1 = event.subscribe();
        let sub2 = event.subscribe();
        let sub3 = event.subscribe();

        // Emit 3 values
        event.emit(1);
        event.emit(2);
        event.emit(3);

        // Give time for delivery
        thread::sleep(Duration::from_millis(50));

        // Each subscriber should receive ALL 3 values
        let mut vals1 = Vec::new();
        let mut vals2 = Vec::new();
        let mut vals3 = Vec::new();

        while let Ok(v) = sub1.recv_timeout(Duration::from_millis(50)) {
            vals1.push(v);
        }
        while let Ok(v) = sub2.recv_timeout(Duration::from_millis(50)) {
            vals2.push(v);
        }
        while let Ok(v) = sub3.recv_timeout(Duration::from_millis(50)) {
            vals3.push(v);
        }

        assert_eq!(
            vals1,
            vec![1, 2, 3],
            "Subscriber 1 should receive all values"
        );
        assert_eq!(
            vals2,
            vec![1, 2, 3],
            "Subscriber 2 should receive all values"
        );
        assert_eq!(
            vals3,
            vec![1, 2, 3],
            "Subscriber 3 should receive all values"
        );
    }

    #[test]
    fn test_dropped_subscriber_does_not_affect_others() {
        let event: Event<i32> = Event::new();

        let sub1 = event.subscribe();
        let sub2 = event.subscribe();

        // Emit first value - both receive it
        event.emit(100);

        // Drop sub1
        drop(sub1);

        // Emit second value - only sub2 should receive it
        event.emit(200);

        thread::sleep(Duration::from_millis(50));

        // sub2 should have both values
        let val1 = sub2.recv_timeout(Duration::from_millis(50)).unwrap();
        let val2 = sub2.recv_timeout(Duration::from_millis(50)).unwrap();
        assert_eq!(val1, 100);
        assert_eq!(val2, 200);
    }

    #[test]
    fn test_event_map() {
        let event: Event<i32> = Event::new();
        let mapped = event.clone().map(|x| x * 2);
        let receiver = mapped.subscribe();

        event.emit(5);
        thread::sleep(Duration::from_millis(50));

        let val = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val, 10);
    }

    #[test]
    fn test_event_map_broadcast() {
        // Verify that mapped events also broadcast to multiple subscribers
        let event: Event<i32> = Event::new();
        let mapped = event.clone().map(|x| x * 2);

        let sub1 = mapped.subscribe();
        let sub2 = mapped.subscribe();

        event.emit(5);
        event.emit(10);
        thread::sleep(Duration::from_millis(50));

        // Both subscribers should receive both mapped values
        let v1_1 = sub1.recv_timeout(Duration::from_millis(100)).unwrap();
        let v1_2 = sub1.recv_timeout(Duration::from_millis(100)).unwrap();
        let v2_1 = sub2.recv_timeout(Duration::from_millis(100)).unwrap();
        let v2_2 = sub2.recv_timeout(Duration::from_millis(100)).unwrap();

        assert_eq!((v1_1, v1_2), (10, 20));
        assert_eq!((v2_1, v2_2), (10, 20));
    }

    #[test]
    fn test_event_filter() {
        let event: Event<i32> = Event::new();
        let filtered = event.clone().filter(|x| *x > 10);
        let receiver = filtered.subscribe();

        event.emit(5); // Should be filtered out
        event.emit(15); // Should pass through
        event.emit(3); // Should be filtered out
        event.emit(20); // Should pass through

        thread::sleep(Duration::from_millis(50));

        let val1 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        let val2 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();

        assert_eq!(val1, 15);
        assert_eq!(val2, 20);

        // Should be no more values
        assert!(receiver.recv_timeout(Duration::from_millis(50)).is_err());
    }

    #[test]
    fn test_event_filter_broadcast() {
        let event: Event<i32> = Event::new();
        let filtered = event.clone().filter(|x| *x > 10);

        let sub1 = filtered.subscribe();
        let sub2 = filtered.subscribe();

        event.emit(5); // filtered
        event.emit(15); // passes
        event.emit(25); // passes

        thread::sleep(Duration::from_millis(50));

        let mut vals1 = Vec::new();
        let mut vals2 = Vec::new();
        while let Ok(v) = sub1.recv_timeout(Duration::from_millis(50)) {
            vals1.push(v);
        }
        while let Ok(v) = sub2.recv_timeout(Duration::from_millis(50)) {
            vals2.push(v);
        }

        assert_eq!(vals1, vec![15, 25]);
        assert_eq!(vals2, vec![15, 25]);
    }

    #[test]
    fn test_event_debounce() {
        let event: Event<i32> = Event::new();
        let debounce_duration = Duration::from_millis(80);
        let debounced = event.clone().debounce(debounce_duration);
        let receiver = debounced.subscribe();

        // Emit first value - should go through immediately
        event.emit(1);
        thread::sleep(Duration::from_millis(20));

        // Emit rapidly - these should be debounced
        event.emit(2);
        thread::sleep(Duration::from_millis(20));
        event.emit(3);

        // Wait for debounce period to fully pass
        thread::sleep(Duration::from_millis(100));

        // Emit after debounce period - should go through
        event.emit(4);
        thread::sleep(Duration::from_millis(50));

        // Collect received values
        let mut received = Vec::new();
        while let Ok(v) = receiver.recv_timeout(Duration::from_millis(100)) {
            received.push(v);
        }

        // Should have at least 2 values (first and one after debounce)
        // Due to timing, might have 2-3 values depending on execution speed
        assert!(
            received.len() >= 2,
            "Expected at least 2 values through debounce, got {:?}",
            received
        );
        assert_eq!(received[0], 1, "First value should always pass through");
        // Last value should be 4 (after debounce period)
        assert!(
            received.contains(&4),
            "Value 4 should pass through after debounce period"
        );
    }

    #[test]
    fn test_subscribe_after_emit_receives_nothing() {
        let event: Event<i32> = Event::new();

        event.emit(1);
        event.emit(2);

        // Subscribe AFTER emissions
        let receiver = event.subscribe();

        // Should not receive previously emitted values
        assert!(receiver.recv_timeout(Duration::from_millis(50)).is_err());

        // But should receive new emissions
        event.emit(3);
        let val = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val, 3);
    }

    #[test]
    fn test_clone_shares_subscribers() {
        let event1: Event<i32> = Event::new();
        let event2 = event1.clone();

        let sub = event1.subscribe();

        // Emit from the clone - subscriber should receive it
        event2.emit(42);

        let val = sub.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val, 42);
    }
}
