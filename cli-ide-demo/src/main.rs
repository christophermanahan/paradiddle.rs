//! A minimal demonstration of the Paradiddle.rs workbench.
//!
//! This program sets up a terminal using `crossterm` and draws two windows
//! side‑by‑side using `ratatui`.  It serves as the starting point for Phase 1
//! of the implementation roadmap【6955392274892†L759-L770】.

use std::error::Error;
use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;

use cli_ide_workbench::window::{EditorWindow, TerminalWindow, Window};

fn main() -> Result<(), Box<dyn Error>> {
    // Enable raw mode so we can control the terminal precisely.
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create our two windows.
    let mut editor = EditorWindow::default();
    let mut term = TerminalWindow::default();

    // Draw a single frame with both windows.
    terminal.draw(|f| {
        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(size);
        editor.render(f, chunks[0]);
        term.render(f, chunks[1]);
    })?;

    // Restore terminal state.
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
