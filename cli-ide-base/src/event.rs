//! An asynchronous event system for the CLI IDE.
//!
//! The [`Event<T>`] type implements a publish‑subscribe pattern where
//! subscribers receive values of type `T`.  It supports functional
//! transformations such as `map`, `filter`, and `debounce` to build event
//! pipelines, similar to VS Code’s event API【6955392274892†L521-L533】.

use std::thread;
use std::time::{Duration, Instant};

use crossbeam::channel::{unbounded, Receiver, Sender};

/// An event stream producing values of type `T`.
#[derive(Clone)]
pub struct Event<T: Clone + Send + 'static> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T: Clone + Send + 'static> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + 'static> Event<T> {
    /// Create a new `Event` with its own channel.
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    /// Publish a value to all subscribers.
    pub fn emit(&self, value: T) {
        let _ = self.sender.send(value);
    }

    /// Get a clone of the internal receiver so you can listen for events.
    pub fn subscribe(&self) -> Receiver<T> {
        self.receiver.clone()
    }

    /// Apply a mapping function to each value in the stream, returning a new event.
    pub fn map<U, F>(self, f: F) -> Event<U>
    where
        U: Clone + Send + 'static,
        F: Fn(T) -> U + Send + Sync + 'static,
    {
        let (sender, receiver) = unbounded();
        let thread_sender = sender.clone();
        let inner_receiver = self.receiver;
        thread::spawn(move || {
            for val in inner_receiver.iter() {
                let mapped = f(val);
                let _ = thread_sender.send(mapped);
            }
        });
        Event { sender, receiver }
    }

    /// Filter events based on a predicate.
    pub fn filter<F>(self, predicate: F) -> Event<T>
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        let (sender, receiver) = unbounded();
        let thread_sender = sender.clone();
        let inner_receiver = self.receiver;
        thread::spawn(move || {
            for val in inner_receiver.iter() {
                if predicate(&val) {
                    let _ = thread_sender.send(val);
                }
            }
        });
        Event { sender, receiver }
    }

    /// Emit values at most once every `duration`.
    pub fn debounce(self, duration: Duration) -> Event<T> {
        let (sender, receiver) = unbounded();
        let thread_sender = sender.clone();
        let inner_receiver = self.receiver;
        thread::spawn(move || {
            let mut last_emit: Option<Instant> = None;
            for val in inner_receiver.iter() {
                let now = Instant::now();
                let should_send = match last_emit {
                    Some(prev) => now.duration_since(prev) >= duration,
                    None => true,
                };
                if should_send {
                    let _ = thread_sender.send(val.clone());
                    last_emit = Some(now);
                }
            }
        });
        Event { sender, receiver }
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

        // Use try_recv with a small timeout to avoid blocking forever
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
    fn test_event_map() {
        let event: Event<i32> = Event::new();
        let mapped = event.clone().map(|x| x * 2);
        let receiver = mapped.subscribe();

        event.emit(5);
        // Give the spawned thread time to process
        thread::sleep(Duration::from_millis(50));

        let val = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val, 10);
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

        // Give the spawned thread time to process
        thread::sleep(Duration::from_millis(50));

        let val1 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        let val2 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();

        assert_eq!(val1, 15);
        assert_eq!(val2, 20);

        // Should be no more values
        assert!(receiver.recv_timeout(Duration::from_millis(50)).is_err());
    }

    #[test]
    fn test_event_debounce() {
        let event: Event<i32> = Event::new();
        let debounce_duration = Duration::from_millis(100);
        let debounced = event.clone().debounce(debounce_duration);
        let receiver = debounced.subscribe();

        // Emit first value - should go through
        event.emit(1);
        thread::sleep(Duration::from_millis(20));

        // Emit second value quickly - should be debounced
        event.emit(2);
        thread::sleep(Duration::from_millis(20));

        // Emit third value quickly - should be debounced
        event.emit(3);

        // Wait for debounce period to pass
        thread::sleep(Duration::from_millis(150));

        // Emit fourth value - should go through (debounce period passed)
        event.emit(4);
        thread::sleep(Duration::from_millis(50));

        // First value should arrive
        let val1 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val1, 1);

        // Fourth value should arrive (after debounce period)
        let val2 = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(val2, 4);
    }
}
