//! TUI screens for each major application view.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub mod agent;
pub mod dataset;
pub mod help;
pub mod settings;
pub mod tools;
pub mod workflow;

/// All available screens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Screen {
    /// AI conversation and live reasoning.
    #[default]
    Agent,
    /// Dataset schema, statistics, and column explorer.
    Dataset,
    /// Interactive workflow DAG editor.
    Workflow,
    /// Tool execution history and runtime metrics.
    Tools,
    /// General, LLM, Pipeline, and Evaluation settings.
    Settings,
    /// Keyboard shortcuts reference.
    Help,
}

impl Screen {
    /// All screens in display order for the tab bar.
    pub const ALL: [Screen; 6] = [
        Screen::Agent,
        Screen::Dataset,
        Screen::Workflow,
        Screen::Tools,
        Screen::Settings,
        Screen::Help,
    ];

    /// Display label for the tab bar.
    pub fn label(&self) -> &str {
        match self {
            Screen::Agent => " Agent ",
            Screen::Dataset => " Dataset ",
            Screen::Workflow => " Workflow ",
            Screen::Tools => " Tools ",
            Screen::Settings => " Settings ",
            Screen::Help => " Help ",
        }
    }

    /// Keyboard shortcut to switch to this screen.
    pub fn key_hint(&self) -> &str {
        match self {
            Screen::Agent => "A",
            Screen::Dataset => "D",
            Screen::Workflow => "W",
            Screen::Tools => "T",
            Screen::Settings => "S",
            Screen::Help => "H",
        }
    }
}

/// Render the active screen (tab bar + content + status bar) into the frame.
pub fn render(frame: &mut Frame, screen: Screen) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_tab_bar(frame, layout[0], screen);
    render_content(frame, layout[1], screen);
    render_status_bar(frame, layout[2], screen);
}

fn render_tab_bar(frame: &mut Frame, area: Rect, active: Screen) {
    let active_style = Style::new()
        .fg(Color::Black)
        .bg(Color::White)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::new().fg(Color::White);

    let mut spans = Vec::new();
    for s in Screen::ALL.iter() {
        let label = if *s == active {
            format!(" {} ", s.key_hint())
        } else {
            format!(" {} ", s.label())
        };
        let style = if *s == active { active_style } else { inactive_style };
        spans.push(ratatui::text::Span::styled(label, style));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_content(frame: &mut Frame, area: Rect, screen: Screen) {
    match screen {
        Screen::Agent => agent::render(frame, area),
        Screen::Dataset => dataset::render(frame, area),
        Screen::Workflow => workflow::render(frame, area),
        Screen::Tools => tools::render(frame, area),
        Screen::Settings => settings::render(frame, area),
        Screen::Help => help::render(frame, area),
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, _screen: Screen) {
    let text = " [Q] Quit  |  [A/D/W/T/S/H] Switch screen  |  [I] AI Suggestions  |  [R] Review";
    frame.render_widget(
        Paragraph::new(text).style(Style::new().add_modifier(Modifier::DIM)),
        area,
    );
}
