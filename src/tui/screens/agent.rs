//! Agent screen — live AI reasoning, tool execution, and conversation.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

/// Render the Agent screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Agent ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = Paragraph::new("AI conversation and live reasoning will appear here.")
        .style(Style::new().add_modifier(Modifier::DIM));
    frame.render_widget(text, inner);
}
