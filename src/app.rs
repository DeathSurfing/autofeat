//! Application state, event loop, and screen routing.

use crate::config::settings::Settings;
use crate::tui::screens::settings;
use crate::tui::screens::Screen;
use crossterm::event::{self, Event, KeyCode};

/// App-level result type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Top-level application state.
pub struct App {
    /// The currently visible screen.
    pub current_screen: Screen,
    /// Application settings.
    pub settings: Settings,
    /// Selected category index on the Settings screen.
    pub settings_category: usize,
    /// Selected field index on the Settings screen.
    pub settings_field: usize,
}

impl App {
    fn new() -> Self {
        Self {
            current_screen: Screen::default(),
            settings: Settings::default(),
            settings_category: 0,
            settings_field: 0,
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
            crate::tui::screens::render(frame, app.current_screen, app);
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q' | 'Q') => break,
                KeyCode::Char(' ') if app.current_screen == Screen::Settings => {
                    handle_settings_action(app);
                }
                KeyCode::Char(c) => {
                    if let Some(screen) = screen_from_key(c) {
                        app.current_screen = screen;
                    }
                }
                KeyCode::Up => {
                    if app.current_screen == Screen::Settings {
                        navigate_settings_field(app, -1);
                    }
                }
                KeyCode::Down => {
                    if app.current_screen == Screen::Settings {
                        navigate_settings_field(app, 1);
                    }
                }
                KeyCode::Left => {
                    if app.current_screen == Screen::Settings {
                        navigate_settings_category(app, -1);
                    } else {
                        app.current_screen = app.current_screen.prev();
                    }
                }
                KeyCode::Right => {
                    if app.current_screen == Screen::Settings {
                        navigate_settings_category(app, 1);
                    } else {
                        app.current_screen = app.current_screen.next();
                    }
                }
                KeyCode::Enter if app.current_screen == Screen::Settings => {
                    handle_settings_action(app);
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

fn field_count(category: usize) -> usize {
    match category {
        0 => 3, // General: Theme, Verbose Logging, Auto Save
        1 => 4, // LLM: Provider, Model, Temperature, Max Tokens
        2 => 5, // Pipeline: Feature Generation, Scaling, Encoding, Polynomial, Datetime
        3 => 2, // Evaluation: Metric, Cross Validation
        4 => 4, // Diagnostics: OpenRouter, API Key, featrs, Rust version
        _ => 0,
    }
}

fn navigate_settings_category(app: &mut App, delta: isize) {
    let len = 5; // CATEGORIES.len()
    let next = (app.settings_category as isize + delta).rem_euclid(len as isize) as usize;
    app.settings_category = next;
    app.settings_field = app.settings_field.min(field_count(next).saturating_sub(1));
}

fn navigate_settings_field(app: &mut App, delta: isize) {
    let count = field_count(app.settings_category);
    if count == 0 {
        return;
    }
    let next = (app.settings_field as isize + delta).rem_euclid(count as isize) as usize;
    app.settings_field = next;
}

fn handle_settings_action(app: &mut App) {
    match app.settings_category {
        0 => settings::handle_general(&mut app.settings, app.settings_field),
        1 => settings::handle_llm(&mut app.settings, app.settings_field),
        2 => settings::handle_pipeline(&mut app.settings, app.settings_field),
        3 => settings::handle_evaluation(&mut app.settings, app.settings_field),
        _ => {}
    }
}
