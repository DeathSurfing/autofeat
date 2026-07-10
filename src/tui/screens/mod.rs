//! TUI screens for each major application view.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::app::App;
use crate::config::theme::Catppuccin;

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
            Screen::Agent => "Agent",
            Screen::Dataset => "Dataset",
            Screen::Workflow => "Workflow",
            Screen::Tools => "Tools",
            Screen::Settings => "Settings",
            Screen::Help => "Help",
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

    /// Move to the next screen (wrapping).
    pub fn next(self) -> Self {
        match self {
            Screen::Agent => Screen::Dataset,
            Screen::Dataset => Screen::Workflow,
            Screen::Workflow => Screen::Tools,
            Screen::Tools => Screen::Settings,
            Screen::Settings => Screen::Help,
            Screen::Help => Screen::Agent,
        }
    }

    /// Move to the previous screen (wrapping).
    pub fn prev(self) -> Self {
        match self {
            Screen::Agent => Screen::Help,
            Screen::Dataset => Screen::Agent,
            Screen::Workflow => Screen::Dataset,
            Screen::Tools => Screen::Workflow,
            Screen::Settings => Screen::Tools,
            Screen::Help => Screen::Settings,
        }
    }
}

/// Render the active screen (tab bar + content + status bar) into the frame.
pub fn render(frame: &mut Frame, screen: Screen, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_tab_bar(frame, layout[0], screen);
    render_content(frame, layout[1], screen, app);
    render_status_bar(frame, layout[2], screen);

    if screen == Screen::Settings && app.settings_popover {
        render_popover(frame, app);
    }
}

fn render_popover(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let popup = centered_rect(60, 50, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(format!(" {} ", app.settings_popover_title))
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::MAUVE).bg(Catppuccin::MANTLE));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Filter input line
    let filter_text = if app.settings_popover_filter.is_empty() {
        "(type to filter...)"
    } else {
        &app.settings_popover_filter
    };
    frame.render_widget(
        Paragraph::new(filter_text)
            .style(Style::new().fg(Catppuccin::TEXT).bg(Catppuccin::SURFACE0)),
        vert[0],
    );

    // Option list
    let items: Vec<ListItem> = app
        .settings_popover_filtered
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == app.settings_popover_selected {
                Style::new()
                    .fg(Catppuccin::CRUST)
                    .bg(Catppuccin::MAUVE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Catppuccin::TEXT)
            };
            ListItem::new(opt.as_str()).style(style)
        })
        .collect();
    frame.render_widget(List::new(items), vert[1]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup[1])[1]
}

fn render_tab_bar(frame: &mut Frame, area: Rect, active: Screen) {
    let active_style = Style::new()
        .fg(Catppuccin::CRUST)
        .bg(Catppuccin::MAUVE)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::new().fg(Catppuccin::SUBTEXT0);

    let mut spans = Vec::new();
    for s in Screen::ALL.iter() {
        let is_active = *s == active;
        let style = if is_active {
            active_style
        } else {
            inactive_style
        };
        let label = format!(" {} ", s.label());
        let key = s.key_hint();
        let span = if is_active {
            ratatui::text::Span::styled(format!(" [{key}] {label} "), style)
        } else {
            ratatui::text::Span::styled(format!(" {label} "), style)
        };
        spans.push(span);
        spans.push(ratatui::text::Span::styled(
            " │ ",
            Style::new().fg(Catppuccin::SURFACE2),
        ));
    }
    spans.pop();
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_content(frame: &mut Frame, area: Rect, screen: Screen, app: &App) {
    match screen {
        Screen::Agent => agent::render(frame, area),
        Screen::Dataset => dataset::render(frame, area),
        Screen::Workflow => workflow::render(frame, area),
        Screen::Tools => tools::render(frame, area),
        Screen::Settings => settings::render(
            frame,
            area,
            &app.settings,
            app.settings_category,
            app.settings_field,
            app.settings_editing,
            &app.settings_edit_buffer,
            &app.api_key_status,
        ),
        Screen::Help => help::render(frame, area),
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, _screen: Screen) {
    let text = " ◄/► Navigate  |  [Q] Quit  |  [I] AI Suggestions  |  [R] Review";
    frame.render_widget(
        Paragraph::new(text).style(Style::new().fg(Catppuccin::OVERLAY0)),
        area,
    );
}
