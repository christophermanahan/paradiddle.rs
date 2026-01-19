//! Performance benchmarks for UI rendering and App core.
//!
//! Run with: `cargo bench -p cli-ide-workbench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use cli_ide_workbench::app::App;
use cli_ide_workbench::input::{AppEvent, AppKey};
use cli_ide_workbench::window::{EditorWindow, TerminalWindow, Window};
use ratatui::backend::TestBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;

/// Benchmark rendering a single EditorWindow.
fn bench_render_editor_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_editor");

    for (width, height) in [(80, 24), (120, 40), (200, 60)] {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let backend = TestBackend::new(w, h);
                let mut terminal = Terminal::new(backend).unwrap();
                let mut editor = EditorWindow::default();

                b.iter(|| {
                    terminal
                        .draw(|f| {
                            let area = f.area();
                            editor.render(f, area);
                        })
                        .unwrap();
                    black_box(&terminal);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark rendering a single TerminalWindow.
fn bench_render_terminal_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_terminal");

    for (width, height) in [(80, 24), (120, 40), (200, 60)] {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let backend = TestBackend::new(w, h);
                let mut terminal = Terminal::new(backend).unwrap();
                let mut term_window = TerminalWindow::default();

                b.iter(|| {
                    terminal
                        .draw(|f| {
                            let area = f.area();
                            term_window.render(f, area);
                        })
                        .unwrap();
                    black_box(&terminal);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark rendering the split layout with both windows.
fn bench_render_split_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_split_layout");

    for (width, height) in [(80, 24), (120, 40), (200, 60)] {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let backend = TestBackend::new(w, h);
                let mut terminal = Terminal::new(backend).unwrap();
                let mut editor = EditorWindow::default();
                let mut term_window = TerminalWindow::default();

                b.iter(|| {
                    terminal
                        .draw(|f| {
                            let size = f.area();
                            let chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(50), Constraint::Percentage(50)]
                                        .as_ref(),
                                )
                                .split(size);
                            editor.render(f, chunks[0]);
                            term_window.render(f, chunks[1]);
                        })
                        .unwrap();
                    black_box(&terminal);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark terminal creation overhead.
fn bench_terminal_creation(c: &mut Criterion) {
    c.bench_function("terminal_creation_80x24", |b| {
        b.iter(|| {
            let backend = TestBackend::new(80, 24);
            let terminal = Terminal::new(backend).unwrap();
            black_box(terminal);
        });
    });
}

/// Benchmark App::handle_event for quit key.
fn bench_app_handle_event_quit(c: &mut Criterion) {
    c.bench_function("app_handle_event_quit", |b| {
        b.iter(|| {
            let mut app = App::new();
            app.handle_event(black_box(AppEvent::Key(AppKey::Q)));
            black_box(&app);
        });
    });
}

/// Benchmark App::handle_event for resize.
fn bench_app_handle_event_resize(c: &mut Criterion) {
    c.bench_function("app_handle_event_resize", |b| {
        let mut app = App::new();
        b.iter(|| {
            app.handle_event(black_box(AppEvent::Resize(100, 50)));
            black_box(&app);
        });
    });
}

/// Benchmark App::handle_event for tab (focus toggle).
fn bench_app_handle_event_tab(c: &mut Criterion) {
    c.bench_function("app_handle_event_tab", |b| {
        let mut app = App::new();
        b.iter(|| {
            app.handle_event(black_box(AppEvent::Key(AppKey::Tab)));
            black_box(&app);
        });
    });
}

/// Benchmark App::render at various sizes.
fn bench_app_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("app_render");

    for (width, height) in [(80, 24), (120, 40), (200, 60)] {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let backend = TestBackend::new(w, h);
                let mut terminal = Terminal::new(backend).unwrap();
                let mut app = App::new();

                b.iter(|| {
                    terminal
                        .draw(|f| {
                            let area = f.area();
                            app.render(f, area);
                        })
                        .unwrap();
                    black_box(&terminal);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark App creation.
fn bench_app_new(c: &mut Criterion) {
    c.bench_function("app_new", |b| {
        b.iter(|| {
            let app = black_box(App::new());
            black_box(app);
        });
    });
}

criterion_group!(
    benches,
    bench_render_editor_window,
    bench_render_terminal_window,
    bench_render_split_layout,
    bench_terminal_creation,
    bench_app_handle_event_quit,
    bench_app_handle_event_resize,
    bench_app_handle_event_tab,
    bench_app_render,
    bench_app_new,
);
criterion_main!(benches);
