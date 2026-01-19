//! Snapshot tests for UI windows.
//!
//! These tests render windows to an offscreen buffer using ratatui's TestBackend
//! and verify the output matches expected patterns. They require no TTY and are
//! fully deterministic for CI.

#[cfg(test)]
mod tests {
    use crate::window::{EditorWindow, TerminalWindow, Window};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::{Constraint, Direction, Layout, Rect};
    use ratatui::Terminal;

    /// Render a window to a buffer and return the buffer content as a string.
    fn render_window_to_string<W: Window>(window: &mut W, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.area();
                window.render(f, area);
            })
            .unwrap();

        buffer_to_string(terminal.backend().buffer())
    }

    /// Render two windows in a horizontal split and return the buffer content.
    fn render_split_layout_to_string(
        left: &mut impl Window,
        right: &mut impl Window,
        width: u16,
        height: u16,
    ) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let size = f.area();
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(size);
                left.render(f, chunks[0]);
                right.render(f, chunks[1]);
            })
            .unwrap();

        buffer_to_string(terminal.backend().buffer())
    }

    /// Convert a ratatui Buffer to a string representation.
    /// Each row is separated by a newline.
    fn buffer_to_string(buffer: &Buffer) -> String {
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
    // Test: EditorWindow renders correctly
    // ============================================================

    #[test]
    fn renders_editor_window_basic() {
        let mut editor = EditorWindow::default();
        let output = render_window_to_string(&mut editor, 40, 10);

        // Verify the title "Editor" appears in the border
        assert!(
            output.contains("Editor"),
            "EditorWindow should have 'Editor' title.\nOutput:\n{}",
            output
        );

        // Verify the welcome message appears
        assert!(
            output.contains("Welcome to Paradiddle.rs!"),
            "EditorWindow should contain welcome message.\nOutput:\n{}",
            output
        );

        // Verify borders are present (corner characters)
        assert!(
            output.contains("┌")
                && output.contains("┐")
                && output.contains("└")
                && output.contains("┘"),
            "EditorWindow should have box-drawing border characters.\nOutput:\n{}",
            output
        );
    }

    #[test]
    fn editor_window_has_horizontal_borders() {
        let mut editor = EditorWindow::default();
        let output = render_window_to_string(&mut editor, 40, 10);

        // Horizontal borders use ─
        assert!(
            output.contains("─"),
            "EditorWindow should have horizontal border characters.\nOutput:\n{}",
            output
        );
    }

    // ============================================================
    // Test: TerminalWindow renders correctly
    // ============================================================

    #[test]
    fn renders_terminal_window_basic() {
        let mut terminal = TerminalWindow::default();
        let output = render_window_to_string(&mut terminal, 40, 10);

        // Verify the title "Terminal" appears
        assert!(
            output.contains("Terminal"),
            "TerminalWindow should have 'Terminal' title.\nOutput:\n{}",
            output
        );

        // Verify placeholder text appears
        assert!(
            output.contains("Terminal output will appear here"),
            "TerminalWindow should contain placeholder text.\nOutput:\n{}",
            output
        );

        // Verify borders are present
        assert!(
            output.contains("┌") && output.contains("┐"),
            "TerminalWindow should have border corners.\nOutput:\n{}",
            output
        );
    }

    // ============================================================
    // Test: Split layout with both windows
    // ============================================================

    #[test]
    fn renders_split_layout_two_windows() {
        let mut editor = EditorWindow::default();
        let mut terminal = TerminalWindow::default();
        let output = render_split_layout_to_string(&mut editor, &mut terminal, 80, 24);

        // Both titles should appear
        assert!(
            output.contains("Editor"),
            "Split layout should contain Editor title.\nOutput:\n{}",
            output
        );
        assert!(
            output.contains("Terminal"),
            "Split layout should contain Terminal title.\nOutput:\n{}",
            output
        );

        // Both content areas should appear
        assert!(
            output.contains("Welcome"),
            "Split layout should show Editor content.\nOutput:\n{}",
            output
        );
        assert!(
            output.contains("output will appear"),
            "Split layout should show Terminal content.\nOutput:\n{}",
            output
        );

        // Verify we have multiple border frames (at least 4 corners for left pane)
        let corner_count = output.matches("┌").count();
        assert!(
            corner_count >= 2,
            "Split layout should have at least 2 top-left corners (one per pane), got {}.\nOutput:\n{}",
            corner_count,
            output
        );
    }

    #[test]
    fn split_layout_panes_are_non_overlapping() {
        let mut editor = EditorWindow::default();
        let mut terminal = TerminalWindow::default();

        let backend = TestBackend::new(80, 24);
        let mut term = Terminal::new(backend).unwrap();

        // Capture the chunks used
        let mut left_rect = Rect::default();
        let mut right_rect = Rect::default();

        term.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);
            left_rect = chunks[0];
            right_rect = chunks[1];
            editor.render(f, chunks[0]);
            terminal.render(f, chunks[1]);
        })
        .unwrap();

        // Verify panes don't overlap
        assert!(
            left_rect.x + left_rect.width <= right_rect.x
                || right_rect.x + right_rect.width <= left_rect.x,
            "Panes should not overlap horizontally. Left: {:?}, Right: {:?}",
            left_rect,
            right_rect
        );

        // Verify both panes have non-zero area
        assert!(
            left_rect.width > 0 && left_rect.height > 0,
            "Left pane should have non-zero dimensions: {:?}",
            left_rect
        );
        assert!(
            right_rect.width > 0 && right_rect.height > 0,
            "Right pane should have non-zero dimensions: {:?}",
            right_rect
        );
    }

    // ============================================================
    // Test: Edge cases
    // ============================================================

    #[test]
    fn editor_window_renders_at_minimum_size() {
        let mut editor = EditorWindow::default();
        // Minimum viable size for a bordered widget
        let output = render_window_to_string(&mut editor, 10, 5);

        // Should still have border corners even at small size
        assert!(
            output.contains("┌") && output.contains("┘"),
            "EditorWindow should render borders even at small size.\nOutput:\n{}",
            output
        );
    }

    #[test]
    fn terminal_window_renders_at_minimum_size() {
        let mut terminal = TerminalWindow::default();
        let output = render_window_to_string(&mut terminal, 10, 5);

        assert!(
            output.contains("┌") && output.contains("┘"),
            "TerminalWindow should render borders even at small size.\nOutput:\n{}",
            output
        );
    }

    #[test]
    fn split_layout_renders_at_various_sizes() {
        // Test that layout works at different terminal sizes
        for (width, height) in [(60, 20), (100, 30), (80, 24)] {
            let mut editor = EditorWindow::default();
            let mut terminal = TerminalWindow::default();
            let output = render_split_layout_to_string(&mut editor, &mut terminal, width, height);

            assert!(
                output.contains("Editor") && output.contains("Terminal"),
                "Split layout at {}x{} should contain both titles.\nOutput:\n{}",
                width,
                height,
                output
            );
        }
    }
}
