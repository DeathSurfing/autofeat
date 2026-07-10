//! Agent screen — live AI reasoning, tool execution, and conversation.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::agent::AgentState;
use crate::config::theme::Catppuccin;

/// Render the Agent screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect, agent: &AgentState) {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    render_conversation(frame, vert[0], agent);
    render_input(frame, vert[1], agent);
}

fn render_conversation(frame: &mut Frame, area: Rect, agent: &AgentState) {
    let block = Block::default()
        .title(" Conversation ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = Vec::new();

    for msg in &agent.messages {
        let role_style = match msg.role {
            "You" => Style::new()
                .fg(Catppuccin::GREEN)
                .add_modifier(Modifier::BOLD),
            "Assistant" => Style::new()
                .fg(Catppuccin::MAUVE)
                .add_modifier(Modifier::BOLD),
            _ => Style::new()
                .fg(Catppuccin::YELLOW)
                .add_modifier(Modifier::BOLD),
        };
        lines.push(Line::styled(format!(" [{}]", msg.role), role_style));
        for paragraph in msg.content.lines() {
            lines.push(Line::styled(
                format!("  {}", paragraph),
                Style::new().fg(Catppuccin::TEXT),
            ));
        }
        lines.push(Line::from(""));
    }

    let visible = inner.height as usize;
    let total = lines.len();
    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    // Auto-scroll: start from the end minus visible height (estimated)
    let scroll = if agent.scroll == usize::MAX {
        total.saturating_sub(visible).min(total)
    } else {
        agent.scroll.min(total.saturating_sub(visible))
    };
    frame.render_widget(paragraph.scroll((scroll as u16, 0)), inner);
}

fn render_input(frame: &mut Frame, area: Rect, agent: &AgentState) {
    let block = Block::default()
        .title(" Input ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = if agent.input.is_empty() {
        "Type your message and press Enter..."
    } else {
        &agent.input
    };
    frame.render_widget(
        Paragraph::new(text).style(Style::new().fg(Catppuccin::TEXT).bg(Catppuccin::SURFACE0)),
        inner,
    );
}
