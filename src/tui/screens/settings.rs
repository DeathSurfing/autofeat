//! Settings screen — General, LLM, Pipeline, Evaluation, and Diagnostics configuration.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::config::settings::{Settings, ThemeVariant};

const CATEGORIES: &[&str] = &["General", "LLM", "Pipeline", "Evaluation", "Diagnostics"];

/// Render the Settings screen.
pub fn render(frame: &mut Frame, area: Rect, settings: &Settings, category: usize, field: usize) {
    let horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(1)])
        .split(area);

    render_categories(frame, horz[0], category);
    render_fields(frame, horz[1], settings, category, field);
}

fn render_categories(frame: &mut Frame, area: Rect, selected: usize) {
    let block = Block::default()
        .title(" Category ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = CATEGORIES
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == selected {
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Color::White)
            };
            ListItem::new(format!(" {}", name)).style(style)
        })
        .collect();

    frame.render_widget(List::new(items), inner);
}

fn render_fields(frame: &mut Frame, area: Rect, settings: &Settings, category: usize, field: usize) {
    let block = Block::default()
        .title(format!(" {} ", CATEGORIES[category]))
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    match category {
        0 => render_general(frame, inner, settings, field),
        1 => render_llm(frame, inner, settings, field),
        2 => render_pipeline(frame, inner, settings, field),
        3 => render_evaluation(frame, inner, settings, field),
        4 => render_diagnostics(frame, inner, settings, field),
        _ => {}
    }
}

fn field_style(selected: bool) -> Style {
    if selected {
        Style::new()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::new().fg(Color::White)
    }
}

fn bool_str(v: bool) -> &'static str {
    if v { "On" } else { "Off" }
}

// ── General ──

fn render_general(frame: &mut Frame, area: Rect, settings: &Settings, field: usize) {
    let labels = ["Theme", "Verbose Logging", "Auto Save"];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let value = match i {
            0 => format!("{:?}", settings.theme_variant),
            1 => bool_str(settings.verbose_logging).to_string(),
            2 => bool_str(settings.auto_save).to_string(),
            _ => unreachable!(),
        };
        let style = field_style(i == field);
        let text = format!(" {}  {}", label, value);
        lines.push(Line::styled(text, style));
        lines.push(Line::from(""));
    }

    frame.render_widget(Paragraph::new(ratatui::text::Text::from(lines)), area);
}

/// Handle interaction for the General category.
pub fn handle_general(settings: &mut Settings, field: usize) {
    match field {
        0 => {
            let idx = ThemeVariant::ALL
                .iter()
                .position(|t| *t == settings.theme_variant)
                .unwrap_or(0);
            settings.theme_variant = ThemeVariant::ALL[(idx + 1) % ThemeVariant::ALL.len()];
        }
        1 => settings.verbose_logging = !settings.verbose_logging,
        2 => settings.auto_save = !settings.auto_save,
        _ => {}
    }
}

// ── LLM ──

fn render_llm(frame: &mut Frame, area: Rect, settings: &Settings, field: usize) {
    let labels = ["Provider", "Model", "Temperature", "Max Tokens"];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let value = match i {
            0 => settings.llm.provider.clone(),
            1 => settings.llm.model.clone(),
            2 => format!("{:.1}", settings.llm.temperature),
            3 => format!("{}", settings.llm.max_tokens),
            _ => unreachable!(),
        };
        let style = field_style(i == field);
        lines.push(Line::styled(format!(" {}  {}", label, value), style));
        lines.push(Line::from(""));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

/// Handle interaction for the LLM category.
pub fn handle_llm(_settings: &mut Settings, _field: usize) {
    // TODO: implement field editing for provider/model/temperature/max_tokens
}

// ── Pipeline ──

fn render_pipeline(frame: &mut Frame, area: Rect, settings: &Settings, field: usize) {
    let labels = [
        "Feature Generation",
        "Scaling",
        "Encoding",
        "Polynomial Features",
        "Datetime Features",
    ];
    let mut lines = Vec::new();

    let values = [
        settings.pipeline.feature_generation,
        settings.pipeline.scaling,
        settings.pipeline.encoding,
        settings.pipeline.polynomial_features,
        settings.pipeline.datetime_features,
    ];

    for (i, label) in labels.iter().enumerate() {
        let style = field_style(i == field);
        lines.push(Line::styled(
            format!(" {}  {}", label, bool_str(values[i])),
            style,
        ));
        lines.push(Line::from(""));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

/// Handle interaction for the Pipeline category.
pub fn handle_pipeline(settings: &mut Settings, field: usize) {
    match field {
        0 => settings.pipeline.feature_generation = !settings.pipeline.feature_generation,
        1 => settings.pipeline.scaling = !settings.pipeline.scaling,
        2 => settings.pipeline.encoding = !settings.pipeline.encoding,
        3 => settings.pipeline.polynomial_features = !settings.pipeline.polynomial_features,
        4 => settings.pipeline.datetime_features = !settings.pipeline.datetime_features,
        _ => {}
    }
}

// ── Evaluation ──

const METRICS: &[&str] = &["RMSE", "MAE", "Accuracy", "F1"];

fn render_evaluation(frame: &mut Frame, area: Rect, settings: &Settings, field: usize) {
    let labels = ["Metric", "Cross Validation"];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let value = match i {
            0 => settings.evaluation.metric.clone(),
            1 => format!("{}", settings.evaluation.cross_validation),
            _ => unreachable!(),
        };
        let style = field_style(i == field);
        lines.push(Line::styled(format!(" {}  {}", label, value), style));
        lines.push(Line::from(""));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

/// Handle interaction for the Evaluation category.
pub fn handle_evaluation(settings: &mut Settings, field: usize) {
    match field {
        0 => {
            let idx = METRICS
                .iter()
                .position(|m| *m == settings.evaluation.metric)
                .unwrap_or(0);
            settings.evaluation.metric = METRICS[(idx + 1) % METRICS.len()].to_string();
        }
        1 => {
            settings.evaluation.cross_validation = match settings.evaluation.cross_validation {
                10 => 2,
                2..=9 => settings.evaluation.cross_validation + 1,
                _ => 5,
            };
        }
        _ => {}
    }
}

// ── Diagnostics ──

fn render_diagnostics(frame: &mut Frame, area: Rect, _settings: &Settings, field: usize) {
    let labels = ["OpenRouter Connection", "API Key Status", "featrs Loaded", "Rust Version"];
    let values = ["—", "—", "✓", "1.96.1"];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let style = Style::new().add_modifier(Modifier::DIM).fg(if i == field {
            Color::White
        } else {
            Color::Gray
        });
        lines.push(Line::styled(
            format!(" {}  {}", label, values[i]),
            style,
        ));
        lines.push(Line::from(""));
    }

    frame.render_widget(Paragraph::new(lines), area);
}
