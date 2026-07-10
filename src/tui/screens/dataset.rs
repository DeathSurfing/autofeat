//! Dataset screen — schema, statistics, distributions, null counts, column explorer.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::config::theme::Catppuccin;
use crate::dataset::Dataset;

/// Render the Dataset screen inside the given area.
pub fn render(frame: &mut Frame, area: Rect, dataset: Option<&Dataset>, selected: usize) {
    let Some(ds) = dataset else {
        let block = Block::default()
            .title(" Dataset ")
            .title_alignment(ratatui::layout::Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::new().fg(Catppuccin::SUBTEXT0));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let text = Paragraph::new("No dataset loaded.\n\nUse --dataset <path.csv> to load one.")
            .style(Style::new().fg(Catppuccin::OVERLAY0));
        frame.render_widget(text, inner);
        return;
    };

    // Top info bar
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(area);

    let info = format!(
        " {}  |  {} rows  |  {} columns  |  ↑↓ scroll columns ",
        ds.name,
        ds.df.height(),
        ds.columns.len()
    );
    frame.render_widget(
        Paragraph::new(info)
            .style(Style::new().fg(Catppuccin::SUBTEXT0))
            .bg(Catppuccin::SURFACE0),
        vert[0],
    );

    // Main split: column list (left) + detail panel (right)
    let horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(vert[1]);

    render_column_list(frame, horz[0], ds, selected);
    render_column_detail(frame, horz[1], ds, selected);
}

fn render_column_list(frame: &mut Frame, area: Rect, ds: &Dataset, selected: usize) {
    let block = Block::default()
        .title(" Columns ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = ds
        .columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let selected = i == selected;
            let style = if selected {
                Style::new()
                    .fg(Catppuccin::CRUST)
                    .bg(Catppuccin::MAUVE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Catppuccin::TEXT)
            };
            let null_flag = if col.null_count > 0 {
                format!(" ⚠{}", col.null_count)
            } else {
                String::new()
            };
            ListItem::new(format!(" {}  {}{}", col.name, col.dtype, null_flag)).style(style)
        })
        .collect();

    frame.render_widget(List::new(items), inner);
}

fn render_column_detail(frame: &mut Frame, area: Rect, ds: &Dataset, selected: usize) {
    let block = Block::default()
        .title(" Details ")
        .borders(Borders::ALL)
        .style(Style::new().fg(Catppuccin::SUBTEXT0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(col) = ds.columns.get(selected) else {
        return;
    };

    let mut lines = Vec::new();
    lines.push(Line::styled(
        format!(" {} ", col.name),
        Style::new()
            .fg(Catppuccin::TEXT)
            .add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(""));
    lines.push(Line::styled(
        format!(" Type       {}", col.dtype),
        Style::new().fg(Catppuccin::SUBTEXT0),
    ));
    lines.push(Line::styled(
        format!(" Non-null   {}", col.non_null_count),
        Style::new().fg(Catppuccin::SUBTEXT0),
    ));
    lines.push(Line::styled(
        format!(" Nulls      {} ({:.1}%)", col.null_count, col.null_pct),
        if col.null_count > 0 {
            Style::new().fg(Catppuccin::PEACH)
        } else {
            Style::new().fg(Catppuccin::SUBTEXT0)
        },
    ));
    lines.push(Line::from(""));

    if let Some(ref stats) = col.stats {
        lines.push(Line::styled(
            " Statistics",
            Style::new()
                .fg(Catppuccin::TEXT)
                .add_modifier(Modifier::BOLD),
        ));
        lines.push(Line::styled(
            format!(" Count     {}", stats.count),
            Style::new().fg(Catppuccin::SUBTEXT0),
        ));
        lines.push(Line::styled(
            format!(" Mean      {:.4}", stats.mean),
            Style::new().fg(Catppuccin::SUBTEXT0),
        ));
        lines.push(Line::styled(
            format!(" Std       {:.4}", stats.std),
            Style::new().fg(Catppuccin::SUBTEXT0),
        ));
        lines.push(Line::styled(
            format!(" Min       {:.4}", stats.min),
            Style::new().fg(Catppuccin::SUBTEXT0),
        ));
        lines.push(Line::styled(
            format!(" Max       {:.4}", stats.max),
            Style::new().fg(Catppuccin::SUBTEXT0),
        ));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}
