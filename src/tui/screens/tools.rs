//! Tools screen — execution history, runtime, outputs, failures.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::config::theme::Catppuccin;
use crate::pipeline::ExecutionResult;

/// Render the Tools screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect, history: &[ExecutionResult]) {
    let block = Block::default()
        .title(" Tools ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if history.is_empty() {
        let text =
            Paragraph::new("No pipeline runs yet.\n\nBuild a workflow and press R to execute it.")
                .style(Style::new().fg(Catppuccin::OVERLAY0));
        frame.render_widget(text, inner);
        return;
    }

    let mut lines = Vec::new();
    lines.push(Line::styled(
        format!(" {} runs", history.len()),
        Style::new()
            .fg(Catppuccin::TEXT)
            .add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(""));

    for (i, result) in history.iter().rev().enumerate() {
        let icon = if result.success { "✓" } else { "✗" };
        let color = if result.success {
            Catppuccin::GREEN
        } else {
            Catppuccin::RED
        };
        lines.push(Line::styled(
            format!(
                " #{}  {}  {} → {}  | {} ms  | {}",
                history.len() - i,
                icon,
                result.output_rows,
                result.output_cols,
                result.duration_ms,
                result.message
            ),
            Style::new().fg(color),
        ));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
