//! A minimal demonstration of the Paradiddle.rs workbench.
//!
//! This program sets up a terminal using `crossterm` and runs an interactive
//! event loop using `ratatui`. Press `q` or `Esc` to quit, `Tab` to switch focus.

use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use cli_ide_workbench::app::App;
use cli_ide_workbench::input::{AppEvent, AppKey};

/// RAII guard for terminal cleanup.
///
/// Ensures the terminal is restored to its original state even if the program
/// panics or returns early with an error.
struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    /// Set up the terminal for TUI rendering.
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    /// Get a mutable reference to the terminal.
    fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best effort cleanup - ignore errors during drop
        let _ = terminal::disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

/// Convert a crossterm key event to our internal AppKey.
fn translate_key(code: KeyCode) -> AppKey {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => AppKey::Q,
        KeyCode::Esc => AppKey::Esc,
        KeyCode::Tab => AppKey::Tab,
        KeyCode::Enter => AppKey::Enter,
        KeyCode::Backspace => AppKey::Backspace,
        KeyCode::Up => AppKey::Up,
        KeyCode::Down => AppKey::Down,
        KeyCode::Left => AppKey::Left,
        KeyCode::Right => AppKey::Right,
        KeyCode::Char(c) => AppKey::Char(c),
        _ => AppKey::Other,
    }
}

/// Run the main application loop.
fn run_app(guard: &mut TerminalGuard, app: &mut App) -> io::Result<()> {
    let terminal = guard.terminal();

    loop {
        // Render the current state
        terminal.draw(|frame| {
            let area = frame.area();
            app.handle_event(AppEvent::Resize(area.width, area.height));
            app.render(frame, area);
        })?;

        // Check if we should quit
        if !app.is_running() {
            break;
        }

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) => {
                    // Only handle key press events (not release)
                    if key_event.kind == KeyEventKind::Press {
                        let app_key = translate_key(key_event.code);
                        app.handle_event(AppEvent::Key(app_key));
                    }
                }
                Event::Resize(width, height) => {
                    app.handle_event(AppEvent::Resize(width, height));
                }
                _ => {
                    // Ignore mouse events and other event types for now
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal with RAII guard for cleanup
    let mut guard = TerminalGuard::new()?;

    // Create the application
    let mut app = App::new();

    // Run the event loop
    run_app(&mut guard, &mut app)?;

    // Guard's Drop impl handles terminal restoration
    Ok(())
}
