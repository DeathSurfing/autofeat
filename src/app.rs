//! Application state, event loop, and screen routing.

use crate::tui::screens::Screen;
use crossterm::event::{self, Event, KeyCode};

/// App-level result type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Top-level application state.
pub struct App {
    /// The currently visible screen.
    pub current_screen: Screen,
}

impl App {
    fn new() -> Self {
        Self {
            current_screen: Screen::default(),
        }
    }
}

/// Run the main TUI event loop.
pub async fn run() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app).await;
    ratatui::restore();
    result
}

async fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            crate::tui::screens::render(frame, app.current_screen);
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q' | 'Q') => break,
                KeyCode::Char(c) => {
                    if let Some(screen) = screen_from_key(c) {
                        app.current_screen = screen;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// Map a char key to the corresponding screen.
fn screen_from_key(c: char) -> Option<Screen> {
    match c.to_ascii_lowercase() {
        'a' => Some(Screen::Agent),
        'd' => Some(Screen::Dataset),
        'w' => Some(Screen::Workflow),
        't' => Some(Screen::Tools),
        's' => Some(Screen::Settings),
        'h' => Some(Screen::Help),
        _ => None,
    }
}
