//! Application state, event loop, and screen routing.

/// App-level result type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Top-level application state.
pub struct App {
    // TODO: shared state — dataset, workflow, history, settings, agent
}

/// Run the main TUI event loop.
pub async fn run() -> Result<()> {
    let mut terminal = ratatui::init();
    let _app = App {};
    let result = run_app(&mut terminal).await;
    ratatui::restore();
    result
}

async fn run_app(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|_frame| {
            // TODO: render current screen
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}
