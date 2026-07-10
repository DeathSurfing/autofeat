//! Help screen — keyboard shortcuts reference.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::config::theme::Catppuccin;

/// Render the Help screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Help ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
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
    let text = Paragraph::new(lines.join("\n")).style(Style::new().fg(Catppuccin::OVERLAY0));
    frame.render_widget(text, inner);
}
