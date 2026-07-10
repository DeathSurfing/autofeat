//! Settings screen — General, LLM, Pipeline, Evaluation, and Diagnostics configuration.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Render the Settings screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Settings ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = Paragraph::new("Settings — General, LLM, Pipeline, Evaluation, Diagnostics.")
        .style(Style::new().add_modifier(Modifier::DIM));
    frame.render_widget(text, inner);
}
