use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    cli::commands::cmd_set,
    tui::app::App,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let files = app.filtered_files();
    let active_path = app
        .state
        .monitors
        .values()
        .next()
        .map(|e| e.wallpaper_path.clone())
        .unwrap_or_default();

    let items: Vec<ListItem> = files
        .iter()
        .map(|entry| {
            let is_active = entry.path.to_string_lossy() == active_path;
            let size_str = format_size(entry.size);
            let indicator = if is_active { " ▶" } else { "  " };
            let style = if is_active {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} {}", indicator, entry.name), style),
                Span::styled(
                    format!("  ({})", size_str),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let title = if app.browser_filter_mode {
        format!(" Browser — filter: {}_ ", app.browser_filter)
    } else if !app.browser_filter.is_empty() {
        format!(" Browser — [{}] ", app.browser_filter)
    } else {
        " Browser ".to_string()
    };

    let visible_len = files.len();
    let selected = app.browser_selected.min(visible_len.saturating_sub(1));
    let mut list_state = ListState::default();
    if !files.is_empty() {
        list_state.select(Some(selected));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let filter_hint = if app.browser_filter_mode {
        format!(" / Filter: {}_ (Esc to clear)", app.browser_filter)
    } else {
        " Press / to filter  |  Enter: set wallpaper  |  a: add to library".to_string()
    };
    let filter_bar = Paragraph::new(filter_hint)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(filter_bar, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    // Only handle key press events, ignore key repeat and release
    // This prevents cmd_set firing dozens of times when Enter is held
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.browser_filter_mode {
        match key.code {
            KeyCode::Esc => {
                app.browser_filter_mode = false;
                app.browser_filter.clear();
                app.browser_selected = 0;
            }
            KeyCode::Enter => {
                app.browser_filter_mode = false;
            }
            KeyCode::Backspace => {
                app.browser_filter.pop();
                app.browser_selected = 0;
            }
            KeyCode::Char(c) => {
                app.browser_filter.push(c);
                app.browser_selected = 0;
            }
            _ => {}
        }
        return Ok(());
    }

    let file_count = app.filtered_files().len();

    match key.code {
        KeyCode::Char('/') => {
            app.browser_filter_mode = true;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.browser_selected > 0 {
                app.browser_selected -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if file_count > 0 && app.browser_selected < file_count - 1 {
                app.browser_selected += 1;
            }
        }
        KeyCode::Enter => {
            let entry_data = app
                .filtered_files()
                .get(app.browser_selected)
                .map(|e| (e.path.to_string_lossy().to_string(), e.name.clone()));

            if let Some((path, name)) = entry_data {
                match cmd_set(&path, None) {
                    Ok(_) => {
                        app.refresh_state()?;
                        app.set_message(format!("Set: {}", name), false);
                    }
                    Err(e) => app.set_message(format!("Error: {}", e), true),
                }
            }
        }
        KeyCode::Char('a') => {
            let entry_data = app
                .filtered_files()
                .get(app.browser_selected)
                .map(|e| (e.path.to_string_lossy().to_string(), e.name.clone()));

            if let Some((path, name)) = entry_data {
                app.library.add(path);
                match app.library.save() {
                    Ok(_) => app.set_message(format!("Added: {}", name), false),
                    Err(e) => app.set_message(format!("Error saving library: {}", e), true),
                }
            }
        }
        KeyCode::Esc => {
            app.browser_filter.clear();
        }
        _ => {}
    }
    Ok(())
}

fn format_size(bytes: u64) -> String {
    const MB: u64 = 1024 * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB { format!("{:.1}G", bytes as f64 / GB as f64) }
    else if bytes >= MB { format!("{:.1}M", bytes as f64 / MB as f64) }
    else { format!("{:.1}K", bytes as f64 / 1024.0) }
}
