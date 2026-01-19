//! Performance benchmarks for the Event system.
//!
//! Run with: `cargo bench -p cli-ide-base`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use cli_ide_base::event::Event;

/// Benchmark event emit with varying subscriber counts.
fn bench_event_emit_to_subscribers(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_emit");

    for subscriber_count in [1, 4, 16, 64] {
        group.throughput(Throughput::Elements(subscriber_count as u64));
        group.bench_with_input(
            BenchmarkId::new("subscribers", subscriber_count),
            &subscriber_count,
            |b, &count| {
                let event: Event<i32> = Event::new();

                // Create subscribers
                let receivers: Vec<_> = (0..count).map(|_| event.subscribe()).collect();

                b.iter(|| {
                    event.emit(black_box(42));
                    // Drain receivers to prevent unbounded queue growth
                    for receiver in &receivers {
                        while receiver.try_recv().is_ok() {}
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark subscribe operation.
fn bench_event_subscribe(c: &mut Criterion) {
    c.bench_function("event_subscribe", |b| {
        let event: Event<i32> = Event::new();

        b.iter(|| {
            let _receiver = event.subscribe();
        });
    });
}

/// Benchmark creating a new Event.
fn bench_event_new(c: &mut Criterion) {
    c.bench_function("event_new", |b| {
        b.iter(|| {
            let _event: Event<i32> = black_box(Event::new());
        });
    });
}

/// Benchmark emit + receive round-trip latency.
fn bench_event_round_trip(c: &mut Criterion) {
    c.bench_function("event_emit_recv_roundtrip", |b| {
        let event: Event<i32> = Event::new();
        let receiver = event.subscribe();

        b.iter(|| {
            event.emit(black_box(42));
            let _ = black_box(receiver.recv().unwrap());
        });
    });
}

/// Benchmark map transformation overhead.
fn bench_event_map(c: &mut Criterion) {
    c.bench_function("event_map_transform", |b| {
        b.iter(|| {
            let event: Event<i32> = Event::new();
            let _mapped = event.map(|x| black_box(x * 2));
        });
    });
}

criterion_group!(
    benches,
    bench_event_emit_to_subscribers,
    bench_event_subscribe,
    bench_event_new,
    bench_event_round_trip,
    bench_event_map,
);
criterion_main!(benches);
