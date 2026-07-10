//! Application state, event loop, and screen routing.

use crate::agent::AgentState;
use crate::cli::Cli;
use crate::config::settings::Settings;
use crate::dataset::Dataset;
use crate::pipeline::ExecutionResult;
use crate::tui::screens::Screen;
use crate::tui::screens::settings;
use crate::workflow::graph::WorkflowGraph;
use crossterm::event::{self, Event, KeyCode};

/// App-level result type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Top-level application state.
pub struct App {
    /// The currently visible screen.
    pub current_screen: Screen,
    /// Application settings.
    pub settings: Settings,

    // Dataset
    /// Loaded dataset (None if not loaded).
    pub dataset: Option<Dataset>,
    /// Selected column index on the Dataset screen.
    pub dataset_selected_column: usize,

    // Settings screen navigation
    /// Selected category index on the Settings screen.
    pub settings_category: usize,
    /// Selected field index on the Settings screen.
    pub settings_field: usize,

    // Text editing mode
    /// Whether the user is editing a text field on the Settings screen.
    pub settings_editing: bool,
    /// Buffer for in-progress text input.
    pub settings_edit_buffer: String,

    // Popover mode
    /// Whether a popover is open on the Settings screen.
    pub settings_popover: bool,
    /// Full option list for the popover.
    pub settings_popover_options: Vec<String>,
    /// Filtered option list for display.
    pub settings_popover_filtered: Vec<String>,
    /// Selected index in the filtered list.
    pub settings_popover_selected: usize,
    /// Filter text typed so far.
    pub settings_popover_filter: String,
    /// The category index that opened this popover.
    pub settings_popover_cat: usize,
    /// The field index that opened this popover.
    pub settings_popover_field: usize,
    /// Title for the popover box.
    pub settings_popover_title: String,

    /// API key connection status (for Diagnostics screen).
    pub api_key_status: String,

    // Agent
    /// Agent conversation state.
    pub agent: AgentState,

    // Workflow
    /// The preprocessing pipeline graph.
    pub workflow: WorkflowGraph,
    /// Currently selected node index in the workflow screen.
    pub workflow_selected: usize,
    /// Whether the add-node popover is open.
    pub workflow_adding: bool,
    /// Selected index in the add-node kind list.
    pub workflow_add_selected: usize,
    /// Whether the user is in move-node mode.
    pub workflow_moving: bool,

    // Pipeline execution
    /// Result of the most recent pipeline run.
    pub last_execution: Option<ExecutionResult>,
    /// History of all pipeline runs.
    pub execution_history: Vec<ExecutionResult>,
}

impl App {
    fn new(cli: Cli) -> Self {
        let settings = Settings::load();
        let dataset = cli
            .dataset
            .as_ref()
            .and_then(|p| Dataset::from_csv(std::path::Path::new(p)).ok());
        Self {
            current_screen: Screen::default(),
            settings,
            dataset,
            dataset_selected_column: 0,
            settings_category: 0,
            settings_field: 0,
            settings_editing: false,
            settings_edit_buffer: String::new(),
            settings_popover: false,
            settings_popover_options: Vec::new(),
            settings_popover_filtered: Vec::new(),
            settings_popover_selected: 0,
            settings_popover_filter: String::new(),
            settings_popover_cat: 0,
            settings_popover_field: 0,
            settings_popover_title: String::new(),
            api_key_status: String::new(),
            agent: AgentState::new(),
            workflow: WorkflowGraph::new(),
            workflow_selected: 0,
            workflow_adding: false,
            workflow_add_selected: 0,
            workflow_moving: false,
            last_execution: None,
            execution_history: Vec::new(),
        }
    }

    fn save_settings(&self) {
        self.settings.save();
    }

    /// Check the API key against OpenRouter and update `api_key_status`.
    pub async fn check_api_key(&mut self) {
        let key = &self.settings.llm.api_key;
        if key.is_empty() {
            self.api_key_status = "Not Configured".into();
            return;
        }
        self.api_key_status = "Checking...".into();
        let client = reqwest::Client::new();
        let resp = client
            .get("https://openrouter.ai/api/v1/auth/key")
            .header("Authorization", format!("Bearer {}", key))
            .send()
            .await;
        self.api_key_status = match resp {
            Ok(r) if r.status().is_success() => "✓ Connected".into(),
            Ok(r) if r.status() == 401 => "✗ Invalid Key".into(),
            Ok(_) => "✗ Error".into(),
            Err(e) => format!("✗ {}", e),
        };
    }
}

/// Run the main TUI event loop.
pub async fn run(cli: Cli) -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(cli);
    let result = run_app(&mut terminal, &mut app).await;
    ratatui::restore();
    result
}

async fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    app.check_api_key().await;

    loop {
        terminal.draw(|frame| {
            crate::tui::screens::render(frame, app.current_screen, app);
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            // Agent screen — input popover vs browse mode
            if app.current_screen == Screen::Agent {
                if app.agent.inputting {
                    // Input popover: all keys go to text entry
                    match key.code {
                        KeyCode::Enter => {
                            let text = app.agent.input.clone();
                            app.agent.send_message(text);
                        }
                        KeyCode::Esc => {
                            app.agent.input.clear();
                            app.agent.inputting = false;
                        }
                        KeyCode::Backspace => {
                            app.agent.input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.agent.input.push(c);
                        }
                        _ => {}
                    }
                } else {
                    // Browse mode: keyboard shortcuts work normally
                    match key.code {
                        KeyCode::Enter => {
                            app.agent.inputting = true;
                        }
                        KeyCode::Up => {
                            app.agent.scroll = app.agent.scroll.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            app.agent.scroll = app.agent.scroll.saturating_add(1);
                        }
                        KeyCode::Char('q' | 'Q') => {
                            app.save_settings();
                            break;
                        }
                        KeyCode::Char(c) => {
                            if let Some(screen) = screen_from_key(c) {
                                app.current_screen = screen;
                            }
                        }
                        _ => {}
                    }
                }
                continue;
            }

            // Workflow screen
            if app.current_screen == Screen::Workflow {
                if app.workflow_adding {
                    // Add-node popover
                    match key.code {
                        KeyCode::Esc => app.workflow_adding = false,
                        KeyCode::Enter => {
                            let kinds = crate::workflow::node::NodeKind::ALL;
                            if let Some(kind) = kinds.get(app.workflow_add_selected) {
                                app.workflow.add_node(*kind);
                            }
                            app.workflow_adding = false;
                        }
                        KeyCode::Up => {
                            app.workflow_add_selected = app.workflow_add_selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            let max = crate::workflow::node::NodeKind::ALL.len().saturating_sub(1);
                            app.workflow_add_selected = (app.workflow_add_selected + 1).min(max);
                        }
                        _ => {}
                    }
                } else if app.workflow_moving {
                    // Move mode
                    match key.code {
                        KeyCode::Esc => app.workflow_moving = false,
                        KeyCode::Up => {
                            let sel = app.workflow_selected;
                            app.workflow.move_up(sel);
                            app.workflow_selected = app.workflow_selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            let sel = app.workflow_selected;
                            app.workflow.move_down(sel);
                            app.workflow_selected = (app.workflow_selected + 1)
                                .min(app.workflow.len().saturating_sub(1));
                        }
                        _ => {}
                    }
                } else {
                    // Browse mode: keyboard shortcuts
                    match key.code {
                        KeyCode::Up => {
                            app.workflow_selected = app.workflow_selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            let max = app.workflow.len().saturating_sub(1);
                            app.workflow_selected = (app.workflow_selected + 1).min(max);
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            app.workflow.toggle(app.workflow_selected);
                        }
                        KeyCode::Char('r' | 'R') => {
                            if let Some(ref df) = app.dataset {
                                app.workflow_moving = false;
                                app.workflow_adding = false;
                                let result =
                                    crate::pipeline::execute::run_pipeline(&app.workflow, &df.df);
                                app.last_execution = Some(result.clone());
                                app.execution_history.push(result);
                            }
                        }
                        KeyCode::Char('a' | 'A') => {
                            app.workflow_adding = true;
                            app.workflow_add_selected = 0;
                        }
                        KeyCode::Char('d' | 'D') => {
                            app.workflow.remove_node(app.workflow_selected);
                            let max = app.workflow.len().saturating_sub(1);
                            app.workflow_selected = app.workflow_selected.min(max);
                        }
                        KeyCode::Char('m' | 'M') => {
                            app.workflow_moving = true;
                        }
                        KeyCode::Char('c' | 'C') => {
                            app.workflow.duplicate(app.workflow_selected);
                        }
                        KeyCode::Char('q' | 'Q') => {
                            app.save_settings();
                            break;
                        }
                        KeyCode::Char(c) => {
                            if let Some(screen) = screen_from_key(c) {
                                app.current_screen = screen;
                            }
                        }
                        _ => {}
                    }
                }
                continue;
            }

            // Popover mode — capture all input for filtering / selecting
            if app.current_screen == Screen::Settings && app.settings_popover {
                match key.code {
                    KeyCode::Esc => popover_close(app),
                    KeyCode::Enter => popover_confirm(app),
                    KeyCode::Up => {
                        if !app.settings_popover_filtered.is_empty() {
                            app.settings_popover_selected =
                                app.settings_popover_selected.saturating_sub(1);
                        }
                    }
                    KeyCode::Down => {
                        let len = app.settings_popover_filtered.len();
                        if len > 0 {
                            app.settings_popover_selected =
                                (app.settings_popover_selected + 1).min(len - 1);
                        }
                    }
                    KeyCode::Backspace => {
                        app.settings_popover_filter.pop();
                        popover_apply_filter(app);
                    }
                    KeyCode::Char(c) => {
                        app.settings_popover_filter.push(c);
                        popover_apply_filter(app);
                    }
                    _ => {}
                }
                continue;
            }

            // Text editing mode on the Settings screen
            if app.current_screen == Screen::Settings && app.settings_editing {
                match key.code {
                    KeyCode::Enter => {
                        let edited_api_key = app.settings_category == 1 && app.settings_field == 4;
                        let val = app.settings_edit_buffer.clone();
                        match (app.settings_category, app.settings_field) {
                            (1, 4) => app.settings.llm.api_key = val,
                            (1, 1) => app.settings.llm.model = val,
                            _ => {}
                        }
                        app.settings_editing = false;
                        app.save_settings();
                        if edited_api_key {
                            app.check_api_key().await;
                        }
                    }
                    KeyCode::Esc => {
                        app.settings_editing = false;
                    }
                    KeyCode::Backspace => {
                        app.settings_edit_buffer.pop();
                    }
                    KeyCode::Char(c) => {
                        app.settings_edit_buffer.push(c);
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Char('q' | 'Q') => {
                    app.save_settings();
                    break;
                }
                KeyCode::Char(' ') if app.current_screen == Screen::Settings => {
                    settings_interact(app);
                }
                KeyCode::Char(c) => {
                    if let Some(screen) = screen_from_key(c) {
                        let entered_settings =
                            screen == Screen::Settings && app.current_screen != Screen::Settings;
                        app.current_screen = screen;
                        if entered_settings {
                            app.check_api_key().await;
                        }
                    }
                }
                KeyCode::Up => {
                    if app.current_screen == Screen::Settings {
                        navigate_settings_field(app, -1);
                    } else if app.current_screen == Screen::Dataset {
                        navigate_dataset_column(app, -1);
                    }
                }
                KeyCode::Down => {
                    if app.current_screen == Screen::Settings {
                        navigate_settings_field(app, 1);
                    } else if app.current_screen == Screen::Dataset {
                        navigate_dataset_column(app, 1);
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
                        let next = app.current_screen.next();
                        app.current_screen = next;
                        if next == Screen::Settings {
                            app.check_api_key().await;
                        }
                    }
                }
                KeyCode::Enter | KeyCode::Tab if app.current_screen == Screen::Settings => {
                    settings_interact(app);
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

// ── Dataset navigation helpers ──

fn navigate_dataset_column(app: &mut App, delta: isize) {
    let count = app.dataset.as_ref().map(|d| d.columns.len()).unwrap_or(0);
    if count == 0 {
        return;
    }
    let next = (app.dataset_selected_column as isize + delta).rem_euclid(count as isize) as usize;
    app.dataset_selected_column = next;
}

// ── Settings navigation helpers ──

fn field_count(category: usize) -> usize {
    match category {
        0 => 2,
        1 => 5,
        2 => 5,
        3 => 2,
        4 => 4,
        _ => 0,
    }
}

fn navigate_settings_category(app: &mut App, delta: isize) {
    let len = 5;
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

// ── Settings interaction dispatch ──

fn settings_interact(app: &mut App) {
    if app.settings_category == 1 && app.settings_field == 4 {
        app.settings_edit_buffer = app.settings.llm.api_key.clone();
        app.settings_editing = true;
        return;
    }

    if let Some((options, title)) =
        settings::field_options(app.settings_category, app.settings_field)
    {
        popover_open(app, options, title);
        return;
    }

    handle_settings_action(app);
}

fn handle_settings_action(app: &mut App) {
    match app.settings_category {
        0 => settings::handle_general(&mut app.settings, app.settings_field),
        1 => settings::handle_llm(&mut app.settings, app.settings_field),
        2 => settings::handle_pipeline(&mut app.settings, app.settings_field),
        3 => settings::handle_evaluation(&mut app.settings, app.settings_field),
        _ => {}
    }
    app.save_settings();
}

// ── Popover logic ──

fn popover_open(app: &mut App, options: Vec<String>, title: &str) {
    let current = match (app.settings_category, app.settings_field) {
        (1, 1) => Some(app.settings.llm.model.clone()),
        (1, 2) => Some(format!("{:.1}", app.settings.llm.temperature)),
        (1, 3) => Some(format!("{}", app.settings.llm.max_tokens)),
        _ => None,
    };

    let selected = current
        .and_then(|cur| options.iter().position(|o| *o == cur))
        .unwrap_or(0);

    app.settings_popover_cat = app.settings_category;
    app.settings_popover_field = app.settings_field;
    app.settings_popover_title = title.to_string();
    app.settings_popover_options = options;
    app.settings_popover_selected = selected;
    app.settings_popover_filter = String::new();
    app.settings_popover_filtered = app.settings_popover_options.clone();
    app.settings_popover = true;
}

fn popover_apply_filter(app: &mut App) {
    let filter = app.settings_popover_filter.to_lowercase();
    app.settings_popover_filtered = app
        .settings_popover_options
        .iter()
        .filter(|opt| fuzzy_match(opt, &filter))
        .cloned()
        .collect();
    if app.settings_popover_selected >= app.settings_popover_filtered.len() {
        app.settings_popover_selected = app.settings_popover_filtered.len().saturating_sub(1);
    }
}

fn popover_confirm(app: &mut App) {
    if app.settings_popover_filtered.is_empty() {
        popover_close(app);
        return;
    }
    let value = &app.settings_popover_filtered[app.settings_popover_selected];

    if app.settings_popover_cat == 1 && app.settings_popover_field == 1 && value == "Custom..." {
        popover_close(app);
        app.settings_edit_buffer = app.settings.llm.model.clone();
        app.settings_editing = true;
        return;
    }

    match (app.settings_popover_cat, app.settings_popover_field) {
        (1, 1) => app.settings.llm.model = value.clone(),
        (1, 2) => {
            if let Ok(t) = value.parse::<f64>() {
                app.settings.llm.temperature = t;
            }
        }
        (1, 3) => {
            if let Ok(n) = value.parse::<u32>() {
                app.settings.llm.max_tokens = n;
            }
        }
        _ => {}
    }
    app.save_settings();
    popover_close(app);
}

fn popover_close(app: &mut App) {
    app.settings_popover = false;
    app.settings_popover_options.clear();
    app.settings_popover_filtered.clear();
    app.settings_popover_filter.clear();
}

/// Simple fuzzy match: all chars in `filter` must appear in order in `text`.
fn fuzzy_match(text: &str, filter: &str) -> bool {
    if filter.is_empty() {
        return true;
    }
    let text = text.to_lowercase();
    let mut fi = 0;
    let fbytes = filter.as_bytes();
    for c in text.chars() {
        if fi < fbytes.len() && c as u8 == fbytes[fi] {
            fi += 1;
        }
    }
    fi == fbytes.len()
}
