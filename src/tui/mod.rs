use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

use super::app::{ActivePanel, App};
use super::panels;
use super::ui;

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new()?;

    let tick_rate = Duration::from_millis(250);
    let state_refresh_rate = Duration::from_secs(2);
    let message_clear_rate = Duration::from_secs(3);

    let mut last_tick = Instant::now();
    let mut last_state_refresh = Instant::now();
    let mut message_shown_at: Option<Instant> = None;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Auto-clear message after 3 seconds
        if let Some(shown) = message_shown_at {
            if shown.elapsed() >= message_clear_rate {
                app.message = None;
                message_shown_at = None;
            }
        }
        // Track when a new message appears
        if app.message.is_some() && message_shown_at.is_none() {
            message_shown_at = Some(Instant::now());
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Global keys (processed regardless of panel)
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        return Ok(());
                    }
                    KeyCode::Char('?') => {
                        app.show_help = !app.show_help;
                    }
                    KeyCode::Esc if app.show_help => {
                        app.show_help = false;
                    }
                    KeyCode::Tab => {
                        app.next_panel();
                        app.show_help = false;
                    }
                    KeyCode::BackTab => {
                        app.prev_panel();
                    }
                    _ => {
                        // Delegate to active panel handler
                        let result = match app.active_panel {
                            ActivePanel::Browser => panels::browser::handle_key(&mut app, key),
                            ActivePanel::Status => panels::status::handle_key(&mut app, key),
                            ActivePanel::Library => panels::library::handle_key(&mut app, key),
                            ActivePanel::Settings => panels::settings::handle_key(&mut app, key),
                        };
                        if let Err(e) = result {
                            app.set_message(format!("Error: {}", e), true);
                            message_shown_at = Some(Instant::now());
                        }
                    }
                }
                // Reset message timer on new message
                if app.message.is_some() && message_shown_at.is_none() {
                    message_shown_at = Some(Instant::now());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        // Periodic state refresh
        if last_state_refresh.elapsed() >= state_refresh_rate {
            let _ = app.refresh_state();
            last_state_refresh = Instant::now();
        }
    }
}
