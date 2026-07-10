//! Dataset screen — schema, statistics, distributions, null counts, column explorer.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Render the Dataset screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Dataset ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = Paragraph::new("Dataset schema, statistics, and column explorer will appear here.")
        .style(Style::new().add_modifier(Modifier::DIM));
    frame.render_widget(text, inner);
}
