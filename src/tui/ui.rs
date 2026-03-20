use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
    Frame,
};

use super::app::{ActivePanel, App};
use super::panels;

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.size();

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    draw_tabs(f, app, root[0]);
    draw_panel(f, app, root[1]);
    draw_status_bar(f, app, root[2]);

    if app.show_help {
        draw_help_overlay(f, app, size);
    }
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let c = app.colors();

    let titles: Vec<Line> = ActivePanel::all()
        .iter()
        .map(|p| Line::from(p.label()))
        .collect();

    let tabs = Tabs::new(titles)
        .select(app.active_panel.index())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" mpwall ")
                .border_style(Style::default().fg(c.border_active)),
        )
        .highlight_style(
            Style::default()
                .fg(c.highlight_fg)
                .bg(c.highlight_bg)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(c.text_muted));

    f.render_widget(tabs, area);
}

fn draw_panel(f: &mut Frame, app: &mut App, area: Rect) {
    match app.active_panel {
        ActivePanel::Browser  => panels::browser::draw(f, app, area),
        ActivePanel::Status   => panels::status::draw(f, app, area),
        ActivePanel::Library  => panels::library::draw(f, app, area),
        ActivePanel::Settings => panels::settings::draw(f, app, area),
    }
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let c = app.colors();

    let (text, style) = match &app.message {
        Some(msg) if app.message_is_error => (
            msg.clone(),
            Style::default().fg(c.danger).add_modifier(Modifier::BOLD),
        ),
        Some(msg) => (
            msg.clone(),
            Style::default().fg(c.success),
        ),
        None => (
            " Tab: switch panel  |  q: quit  |  ?: help".to_string(),
            Style::default().fg(c.status_bar_fg),
        ),
    };

    let bar = Paragraph::new(text).style(style);
    f.render_widget(bar, area);
}

fn draw_help_overlay(f: &mut Frame, app: &App, area: Rect) {
    let c = app.colors();

    let width  = 48u16.min(area.width.saturating_sub(4));
    let height = 18u16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let popup = Rect::new(x, y, width, height);

    f.render_widget(Clear, popup);

    let help_text = vec![
        Line::from(vec![Span::styled(
            " Keybindings",
            Style::default().fg(c.text_primary).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(Span::styled(" Tab / Shift+Tab   Switch panel",     Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" ↑ ↓               Navigate list",     Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" Enter             Set wallpaper / confirm", Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" /                 Filter (Browser)",   Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" Esc               Cancel filter / close help", Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" a                 Add to Library",     Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" d                 Remove from Library", Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" e                 Edit field (Settings)", Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" s                 Save settings",       Style::default().fg(c.text_primary))),
        Line::from(""),
        Line::from(Span::styled(" ?                 Toggle this help",    Style::default().fg(c.text_primary))),
        Line::from(Span::styled(" q                 Quit",                Style::default().fg(c.text_primary))),
        Line::from(""),
        Line::from(Span::styled(
            " Press ? or Esc to close",
            Style::default().fg(c.text_muted),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(c.help_border))
        .title(Span::styled(" Help ", Style::default().fg(c.title).add_modifier(Modifier::BOLD)));

    let para = Paragraph::new(help_text).block(block);
    f.render_widget(para, popup);
}

#[allow(dead_code)]
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1]
}
