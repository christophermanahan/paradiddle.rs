//! Integration tests for the App core.
//!
//! These tests drive the App via AppEvent without requiring a TTY,
//! verifying state transitions and rendering output.

use cli_ide_workbench::app::{App, FocusedPane};
use cli_ide_workbench::input::{AppEvent, AppKey};
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;

/// Helper to render the app to a string buffer.
fn render_app_to_string(app: &mut App, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            let area = frame.area();
            app.render(frame, area);
        })
        .unwrap();

    buffer_to_string(terminal.backend().buffer())
}

/// Convert a ratatui Buffer to a string.
fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
    let area = buffer.area;
    let mut result = String::new();

    for y in 0..area.height {
        for x in 0..area.width {
            let cell = buffer.cell((x, y)).unwrap();
            result.push_str(cell.symbol());
        }
        result.push('\n');
    }

    result
}

// ============================================================
// State Transition Tests
// ============================================================

#[test]
fn app_starts_running() {
    let app = App::new();
    assert!(app.is_running(), "App should start in running state");
}

#[test]
fn app_quits_on_q_key() {
    let mut app = App::new();
    assert!(app.is_running());

    app.handle_event(AppEvent::Key(AppKey::Q));

    assert!(!app.is_running(), "App should quit after Q key");
}

#[test]
fn app_quits_on_esc_key() {
    let mut app = App::new();
    assert!(app.is_running());

    app.handle_event(AppEvent::Key(AppKey::Esc));

    assert!(!app.is_running(), "App should quit after Esc key");
}

#[test]
fn app_handles_resize_event() {
    let mut app = App::new();
    assert_eq!(app.size(), (80, 24), "Default size should be 80x24");

    app.handle_event(AppEvent::Resize(120, 40));

    assert_eq!(app.size(), (120, 40), "Size should update after resize");
}

#[test]
fn app_toggles_focus_on_tab() {
    let mut app = App::new();
    assert_eq!(app.focused(), FocusedPane::Editor);

    app.handle_event(AppEvent::Key(AppKey::Tab));
    assert_eq!(app.focused(), FocusedPane::Terminal);

    app.handle_event(AppEvent::Key(AppKey::Tab));
    assert_eq!(app.focused(), FocusedPane::Editor);
}

#[test]
fn app_ignores_other_keys() {
    let mut app = App::new();
    let running_before = app.is_running();
    let focused_before = app.focused();
    let size_before = app.size();

    // Send various non-action keys
    app.handle_event(AppEvent::Key(AppKey::Char('a')));
    app.handle_event(AppEvent::Key(AppKey::Up));
    app.handle_event(AppEvent::Key(AppKey::Other));

    assert_eq!(app.is_running(), running_before);
    assert_eq!(app.focused(), focused_before);
    assert_eq!(app.size(), size_before);
}

// ============================================================
// Render Tests (Non-TTY)
// ============================================================

#[test]
fn app_renders_both_windows() {
    let mut app = App::new();
    let output = render_app_to_string(&mut app, 80, 24);

    assert!(
        output.contains("Editor"),
        "Rendered output should contain Editor title.\nOutput:\n{}",
        output
    );
    assert!(
        output.contains("Terminal"),
        "Rendered output should contain Terminal title.\nOutput:\n{}",
        output
    );
}

#[test]
fn app_renders_window_content() {
    let mut app = App::new();
    let output = render_app_to_string(&mut app, 80, 24);

    assert!(
        output.contains("Welcome"),
        "Rendered output should contain Editor welcome text.\nOutput:\n{}",
        output
    );
    assert!(
        output.contains("output will appear"),
        "Rendered output should contain Terminal placeholder.\nOutput:\n{}",
        output
    );
}

#[test]
fn app_renders_after_resize() {
    let mut app = App::new();

    // Resize to larger dimensions
    app.handle_event(AppEvent::Resize(100, 30));

    let output = render_app_to_string(&mut app, 100, 30);

    // Should still render both windows correctly
    assert!(
        output.contains("Editor") && output.contains("Terminal"),
        "Resized app should still render both windows.\nOutput:\n{}",
        output
    );
}

#[test]
fn app_renders_at_small_size() {
    let mut app = App::new();
    let output = render_app_to_string(&mut app, 40, 10);

    // Should render without panicking and have some content
    assert!(
        output.contains("┌"),
        "Small render should still have border.\nOutput:\n{}",
        output
    );
}

// ============================================================
// Layout Tests
// ============================================================

#[test]
fn app_layout_splits_evenly() {
    let app = App::new();
    let area = Rect::new(0, 0, 80, 24);

    let (left, right) = app.layout_rects(area);

    // Each pane should be roughly half the width
    assert_eq!(left.width, 40, "Left pane should be 40 wide");
    assert_eq!(right.width, 40, "Right pane should be 40 wide");

    // Panes should not overlap
    assert!(
        left.x + left.width <= right.x,
        "Panes should not overlap: left ends at {}, right starts at {}",
        left.x + left.width,
        right.x
    );
}

#[test]
fn app_layout_uses_full_height() {
    let app = App::new();
    let area = Rect::new(0, 0, 80, 24);

    let (left, right) = app.layout_rects(area);

    assert_eq!(left.height, 24, "Left pane should use full height");
    assert_eq!(right.height, 24, "Right pane should use full height");
}

// ============================================================
// Event Sequence Tests
// ============================================================

#[test]
fn app_handles_event_sequence() {
    let mut app = App::new();

    // Resize
    app.handle_event(AppEvent::Resize(100, 50));
    assert_eq!(app.size(), (100, 50));

    // Toggle focus
    app.handle_event(AppEvent::Key(AppKey::Tab));
    assert_eq!(app.focused(), FocusedPane::Terminal);

    // Random key (no effect)
    app.handle_event(AppEvent::Key(AppKey::Char('x')));
    assert!(app.is_running());
    assert_eq!(app.focused(), FocusedPane::Terminal);

    // Quit
    app.handle_event(AppEvent::Key(AppKey::Q));
    assert!(!app.is_running());
}

#[test]
fn app_multiple_resizes() {
    let mut app = App::new();

    let sizes = [(100, 50), (80, 24), (120, 40), (60, 20)];

    for (w, h) in sizes {
        app.handle_event(AppEvent::Resize(w, h));
        assert_eq!(
            app.size(),
            (w, h),
            "Size should match after resize to {}x{}",
            w,
            h
        );
    }
}
