//! Workflow screen — interactive DAG editor with add, delete, move, edit, disable, duplicate.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::config::theme::Catppuccin;
use crate::pipeline::ExecutionResult;
use crate::workflow::graph::WorkflowGraph;

/// Render the Workflow screen inside the given area.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    workflow: &WorkflowGraph,
    selected: usize,
    execution: Option<&ExecutionResult>,
) {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    render_pipeline(frame, vert[0], workflow, selected, execution);
    render_hint(frame, vert[1]);
}

fn render_pipeline(
    frame: &mut Frame,
    area: Rect,
    workflow: &WorkflowGraph,
    selected: usize,
    execution: Option<&ExecutionResult>,
) {
    let block = Block::default()
        .title(" Pipeline ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = Vec::new();

    if workflow.is_empty() {
        lines.push(Line::styled(
            "  (empty — press A to add a node)",
            Style::new().fg(Catppuccin::OVERLAY0),
        ));
    }

    for (i, node) in workflow.nodes.iter().enumerate() {
        let prefix = if i == selected { " >" } else { "  " };
        let status = if node.enabled { "✓" } else { "✗" };
        let style = if i == selected {
            Style::new()
                .fg(Catppuccin::CRUST)
                .bg(Catppuccin::MAUVE)
                .add_modifier(Modifier::BOLD)
        } else if !node.enabled {
            Style::new().fg(Catppuccin::OVERLAY0)
        } else {
            Style::new().fg(Catppuccin::TEXT)
        };
        lines.push(Line::styled(
            format!(" {} [{}] {}", prefix, status, node.kind.label()),
            style,
        ));
        lines.push(Line::from("  │"));
    }

    if !workflow.is_empty() {
        lines.push(Line::styled(
            "  ▼ Output",
            Style::new().fg(Catppuccin::OVERLAY2),
        ));
    }

    // Execution result
    if let Some(result) = execution {
        lines.push(Line::from(""));
        let status_icon = if result.success { "✓" } else { "✗" };
        let status_color = if result.success {
            Catppuccin::GREEN
        } else {
            Catppuccin::RED
        };
        lines.push(Line::styled(
            format!(
                " {} {}  {} → {}  ({} ms)",
                status_icon,
                result.message,
                result.output_rows,
                result.output_cols,
                result.duration_ms
            ),
            Style::new().fg(status_color),
        ));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_hint(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            " ↑↓ select  |  Enter toggle  |  A add  |  D delete  |  M move  |  C dup  |  R run",
        )
        .style(Style::new().fg(Catppuccin::OVERLAY0)),
        area,
    );
}
