//! Settings screen — General, LLM, Pipeline, Evaluation, and Diagnostics configuration.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::config::settings::Settings;
use crate::config::theme::Catppuccin;

const CATEGORIES: &[&str] = &["General", "LLM", "Pipeline", "Evaluation", "Diagnostics"];

#[allow(clippy::too_many_arguments)]
/// Render the Settings screen.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    settings: &Settings,
    category: usize,
    field: usize,
    editing: bool,
    edit_buffer: &str,
    api_key_status: &str,
) {
    let horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(1)])
        .split(area);

    render_categories(frame, horz[0], category);
    render_fields(
        frame,
        horz[1],
        settings,
        category,
        field,
        editing,
        edit_buffer,
        api_key_status,
    );
}

fn render_categories(frame: &mut Frame, area: Rect, selected: usize) {
    let block = Block::default()
        .title(" Category ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let active_style = Style::new()
        .fg(Catppuccin::CRUST)
        .bg(Catppuccin::MAUVE)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::new().fg(Catppuccin::TEXT);

    let items: Vec<ListItem> = CATEGORIES
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == selected {
                active_style
            } else {
                inactive_style
            };
            ListItem::new(format!(" {}", name)).style(style)
        })
        .collect();

    frame.render_widget(List::new(items), inner);
}

#[allow(clippy::too_many_arguments)]
fn render_fields(
    frame: &mut Frame,
    area: Rect,
    settings: &Settings,
    category: usize,
    field: usize,
    editing: bool,
    edit_buffer: &str,
    api_key_status: &str,
) {
    let block = Block::default()
        .title(format!(" {} ", CATEGORIES[category]))
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    match category {
        0 => render_general(frame, inner, settings, field),
        1 => render_llm(frame, inner, settings, field, editing, edit_buffer),
        2 => render_pipeline(frame, inner, settings, field),
        3 => render_evaluation(frame, inner, settings, field),
        4 => render_diagnostics(frame, inner, field, api_key_status),
        _ => {}
    }
}

fn selected_style() -> Style {
    Style::new()
        .fg(Catppuccin::CRUST)
        .bg(Catppuccin::MAUVE)
        .add_modifier(Modifier::BOLD)
}

fn inactive_style() -> Style {
    Style::new().fg(Catppuccin::TEXT)
}

fn bool_str(v: bool) -> &'static str {
    if v { "On" } else { "Off" }
}

// ── General ──

fn render_general(frame: &mut Frame, area: Rect, settings: &Settings, field: usize) {
    let labels = ["Verbose Logging", "Auto Save"];
    let values = [settings.verbose_logging, settings.auto_save];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let style = if i == field {
            selected_style()
        } else {
            inactive_style()
        };
        lines.push(Line::styled(
            format!(" {}  {}", label, bool_str(values[i])),
            style,
        ));
        lines.push(Line::from(""));
    }
    frame.render_widget(Paragraph::new(lines), area);
}

/// Handle interaction for the General category.
pub fn handle_general(settings: &mut Settings, field: usize) {
    match field {
        0 => settings.verbose_logging = !settings.verbose_logging,
        1 => settings.auto_save = !settings.auto_save,
        _ => {}
    }
}

/// Get selectable options for a multi-option field with >5 choices.
/// Returns `(options, field_label)` or `None` if the field doesn't qualify.
pub fn field_options(cat: usize, field: usize) -> Option<(Vec<String>, &'static str)> {
    match (cat, field) {
        (1, 1) => Some((MODELS.iter().map(|s| s.to_string()).collect(), "Model")),
        (1, 2) => Some((
            TEMPS.iter().map(|t| format!("{:.1}", t)).collect(),
            "Temperature",
        )),
        (1, 3) => Some((TOKENS.iter().map(|t| t.to_string()).collect(), "Max Tokens")),
        _ => None,
    }
}

// ── LLM ──

const PROVIDERS: &[&str] = &["openrouter", "openai", "anthropic", "cohere", "google"];

const MODELS: &[&str] = &[
    "gpt-4o",
    "gpt-4o-mini",
    "gpt-4.1",
    "gpt-4.1-mini",
    "gpt-4.1-nano",
    "claude-3-opus",
    "claude-3-sonnet",
    "claude-3-haiku",
    "claude-4-opus",
    "claude-4-sonnet",
    "gemini-1.5-pro",
    "gemini-1.5-flash",
    "gemini-2.0-flash",
    "gemini-2.0-pro",
    "command-r-plus",
    "Custom...",
];

const TEMPS: &[f64] = &[0.0, 0.1, 0.3, 0.5, 0.7, 1.0, 1.5, 2.0];

const TOKENS: &[u32] = &[1024, 2048, 4096, 8192, 16384, 32768];

fn mask_key(key: &str) -> String {
    if key.is_empty() {
        "(not set)".into()
    } else {
        "••••••••".into()
    }
}

fn render_llm(
    frame: &mut Frame,
    area: Rect,
    settings: &Settings,
    field: usize,
    editing: bool,
    edit_buffer: &str,
) {
    let labels = ["Provider", "Model", "Temperature", "Max Tokens", "API Key"];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let is_selected = i == field;
        let style = if is_selected {
            selected_style()
        } else {
            inactive_style()
        };

        let value = match i {
            0 => settings.llm.provider.clone(),
            1 => {
                if editing && is_selected {
                    if edit_buffer.is_empty() {
                        "(type model name and press Enter)".into()
                    } else {
                        edit_buffer.into()
                    }
                } else {
                    settings.llm.model.clone()
                }
            }
            2 => format!("{:.1}", settings.llm.temperature),
            3 => format!("{}", settings.llm.max_tokens),
            4 => {
                if editing && is_selected {
                    if edit_buffer.is_empty() {
                        "(type and press Enter)".into()
                    } else {
                        edit_buffer.into()
                    }
                } else {
                    mask_key(&settings.llm.api_key)
                }
            }
            _ => unreachable!(),
        };

        lines.push(Line::styled(format!(" {}  {}", label, value), style));
        lines.push(Line::from(""));
    }
    frame.render_widget(Paragraph::new(lines), area);
}

/// Handle interaction for the LLM category.
pub fn handle_llm(settings: &mut Settings, field: usize) {
    match field {
        0 => {
            let idx = PROVIDERS
                .iter()
                .position(|p| *p == settings.llm.provider)
                .unwrap_or(0);
            settings.llm.provider = PROVIDERS[(idx + 1) % PROVIDERS.len()].to_string();
        }
        1 => {
            let idx = MODELS
                .iter()
                .position(|m| *m == settings.llm.model)
                .unwrap_or(0);
            settings.llm.model = MODELS[(idx + 1) % MODELS.len()].to_string();
        }
        2 => {
            let idx = TEMPS
                .iter()
                .position(|t| (*t - settings.llm.temperature).abs() < 0.01)
                .unwrap_or(0);
            settings.llm.temperature = TEMPS[(idx + 1) % TEMPS.len()];
        }
        3 => {
            let idx = TOKENS
                .iter()
                .position(|t| *t == settings.llm.max_tokens)
                .unwrap_or(0);
            settings.llm.max_tokens = TOKENS[(idx + 1) % TOKENS.len()];
        }
        // API Key is handled via text editing — see app.rs
        _ => {}
    }
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
    let values = [
        settings.pipeline.feature_generation,
        settings.pipeline.scaling,
        settings.pipeline.encoding,
        settings.pipeline.polynomial_features,
        settings.pipeline.datetime_features,
    ];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let style = if i == field {
            selected_style()
        } else {
            inactive_style()
        };
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
    let values: [String; 2] = [
        settings.evaluation.metric.clone(),
        format!("{}", settings.evaluation.cross_validation),
    ];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let style = if i == field {
            selected_style()
        } else {
            inactive_style()
        };
        lines.push(Line::styled(format!(" {}  {}", label, values[i]), style));
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

fn render_diagnostics(frame: &mut Frame, area: Rect, field: usize, api_key_status: &str) {
    let labels = [
        "OpenRouter Connection",
        "API Key Status",
        "featrs Loaded",
        "Rust Version",
    ];
    let values = [api_key_status, "", "✓", "1.96.1"];
    let mut lines = Vec::new();

    for (i, label) in labels.iter().enumerate() {
        let style = if i == field {
            Style::new().fg(Catppuccin::TEXT)
        } else {
            Style::new().fg(Catppuccin::OVERLAY0)
        };
        lines.push(Line::styled(format!(" {}  {}", label, values[i]), style));
        lines.push(Line::from(""));
    }
    frame.render_widget(Paragraph::new(lines), area);
}
