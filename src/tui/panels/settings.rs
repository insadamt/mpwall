use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    cli::commands::{cmd_disable, cmd_enable, cmd_set},
    core::config::Config,
    tui::app::App,
};

const FIELD_COUNT: usize = 5;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let se = &app.settings_edit;

    let fields: Vec<(&str, String, bool)> = vec![
        ("Wallpaper Directory", se.wallpaper_dir.clone(), true),
        ("Volume (0-100)", se.volume.clone(), true),
        ("Speed (e.g. 1.0)", se.speed.clone(), true),
        ("Loop Video", if se.loop_video { "on".to_string() } else { "off".to_string() }, false),
        ("Autostart on login", if se.autostart { "enabled".to_string() } else { "disabled".to_string() }, false),
    ];

    let items: Vec<ListItem> = fields
        .iter()
        .enumerate()
        .map(|(i, (label, value, text_editable))| {
            let is_selected = i == se.active_field;
            let is_editing = is_selected && se.editing && *text_editable;
            let value_display = if is_editing { format!("{}_", value) } else { value.clone() };

            let label_style = if is_selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let is_on = value == "on" || value == "enabled";
            let value_style = if is_editing {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)
            } else if !text_editable {
                if is_on { Style::default().fg(Color::Green) } else { Style::default().fg(Color::Red) }
            } else if is_selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:<24} ", label), label_style),
                Span::styled(value_display, value_style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(se.active_field));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Settings ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Single-line hint bar — no config path to avoid overflow
    let hints = Paragraph::new(" ↑↓/jk: navigate  |  Enter: edit/toggle  |  s: save & apply  |  q: quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hints, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    if app.settings_edit.editing {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                app.settings_edit.editing = false;
            }
            KeyCode::Backspace => {
                match app.settings_edit.active_field {
                    0 => { app.settings_edit.wallpaper_dir.pop(); }
                    1 => { app.settings_edit.volume.pop(); }
                    2 => { app.settings_edit.speed.pop(); }
                    _ => {}
                }
            }
            KeyCode::Char(c) => {
                match app.settings_edit.active_field {
                    0 => app.settings_edit.wallpaper_dir.push(c),
                    1 => app.settings_edit.volume.push(c),
                    2 => app.settings_edit.speed.push(c),
                    _ => {}
                }
            }
            _ => {}
        }
        return Ok(());
    }

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.settings_edit.active_field > 0 {
                app.settings_edit.active_field -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.settings_edit.active_field < FIELD_COUNT - 1 {
                app.settings_edit.active_field += 1;
            }
        }
        KeyCode::Char('e') | KeyCode::Enter => {
            match app.settings_edit.active_field {
                0 | 1 | 2 => { app.settings_edit.editing = true; }
                3 => {
                    app.settings_edit.loop_video = !app.settings_edit.loop_video;
                }
                4 => {
                    let enabling = !app.settings_edit.autostart;
                    if enabling {
                        match cmd_enable() {
                            Ok(_) => {
                                app.settings_edit.autostart = true;
                                app.set_message("Autostart enabled", false);
                            }
                            Err(e) => app.set_message(format!("Autostart error: {}", e), true),
                        }
                    } else {
                        match cmd_disable() {
                            Ok(_) => {
                                app.settings_edit.autostart = false;
                                app.set_message("Autostart disabled", false);
                            }
                            Err(e) => app.set_message(format!("Autostart error: {}", e), true),
                        }
                    }
                    let _ = app.refresh_state();
                }
                _ => {}
            }
        }
        KeyCode::Char('s') => {
            match save_and_apply(app) {
                Ok(msg) => app.set_message(msg, false),
                Err(e) => app.set_message(format!("Error: {}", e), true),
            }
        }
        _ => {}
    }
    Ok(())
}

fn save_and_apply(app: &mut App) -> Result<String> {
    let volume: u8 = app.settings_edit.volume.parse().map_err(|_| {
        anyhow::anyhow!("Invalid volume '{}' — must be 0-100", app.settings_edit.volume)
    })?;
    let speed: f32 = app.settings_edit.speed.parse().map_err(|_| {
        anyhow::anyhow!("Invalid speed '{}' — must be a number like 1.0", app.settings_edit.speed)
    })?;
    if volume > 100 { anyhow::bail!("Volume must be between 0 and 100"); }
    if speed <= 0.0 { anyhow::bail!("Speed must be greater than 0"); }

    let new_config = Config {
        schema_version: app.config.schema_version,
        wallpaper_dir: app.settings_edit.wallpaper_dir.clone(),
        mpvpaper_flags: String::new(),
        loop_video: app.settings_edit.loop_video,
        volume,
        speed,
    };

    new_config.save()?;
    app.config = new_config;
    app.browser_files = App::scan_files(&app.config.wallpaper_dir);
    app.browser_selected = 0;

    let active_wallpapers: Vec<(String, String)> = app
        .state
        .monitors
        .iter()
        .filter(|(_, e)| !e.wallpaper_path.is_empty())
        .map(|(mon, e)| (mon.clone(), e.wallpaper_path.clone()))
        .collect();

    let mut applied = 0;
    for (mon, path) in active_wallpapers {
        if cmd_set(&path, Some(&mon)).is_ok() {
            applied += 1;
        }
    }
    app.refresh_state()?;

    if applied > 0 {
        Ok(format!("Settings saved & applied to {} monitor(s)", applied))
    } else {
        Ok("Settings saved".to_string())
    }
}
