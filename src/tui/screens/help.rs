//! Help screen — keyboard shortcuts reference.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Render the Help screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Help ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = [
        "Keyboard Shortcuts:",
        "",
        "  A        Agent screen",
        "  D        Dataset screen",
        "  W        Workflow screen",
        "  T        Tools screen",
        "  S        Settings screen",
        "  H        Help screen",
        "  Enter    Edit node",
        "  A        Add node",
        "  M        Move node",
        "  D        Delete node",
        "  Space    Disable / enable node",
        "  I        AI suggestions",
        "  R        Review pipeline",
        "  Ctrl+S   Save",
        "  Q        Quit",
    ];
    let text = Paragraph::new(lines.join("\n")).style(Style::new().add_modifier(Modifier::DIM));
    frame.render_widget(text, inner);
}
