use crate::tracker;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub async fn run(pool: &sqlx::PgPool) -> io::Result<()> {
    enable_raw_mode()?; // raw mode rather than cooked mode, doesn't require an enter at the end for everything
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // ? stops it from crashing

    let backend = CrosstermBackend::new(stdout); // talks from ratatui to terminal
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut active_session = tracker::session::get_active_session(pool).await;
    let mut daily_summary = tracker::session::get_daily_summary(pool).await;
    let mut weekly_summary = tracker::session::get_weekly_summary(pool).await;

    // event loop prompting us for actions
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let status_text = match &active_session {
                Some((name, started_at)) => {
                    let time_elapsed = chrono::Utc::now() - *started_at; // c style deref since
                                                                         // active_session is
                                                                         // behind a & reference

                    let secs = time_elapsed.num_seconds();
                    let hours = secs / 3600;
                    let mins = (secs % 3600) / 60;
                    let seconds = secs % 60;
                    format!("Tracking: {} | {}h {}m {}s", name, hours, mins, seconds)
                }
                None => String::from("No active session"),
            };

            let status_block = Paragraph::new(status_text).block(
                Block::default()
                    .title("productiviCLI")
                    .borders(Borders::ALL),
            );
            f.render_widget(status_block, chunks[0]);

            let daily_total_secs: i64 = daily_summary.iter().map(|(_, s)| s).sum();
            let mut summary_lines = daily_summary
                .iter()
                .map(|(name, total_secs)| {
                    let hours = total_secs / 3600;
                    let mins = (total_secs % 3600) / 60;
                    format!("{:<20} {}h {}m", name, hours, mins)
                })
                .collect::<Vec<_>>();
            if !summary_lines.is_empty() {
                let total_h = daily_total_secs / 3600;
                let total_m = (daily_total_secs % 3600) / 60;
                summary_lines.push(format!("\n{}h {:02}m of deep work today!", total_h, total_m));
            }
            let summary_text = summary_lines.join("\n");

            let summary_block = Paragraph::new(if summary_text.is_empty() {
                String::from("No sessions today")
            } else {
                summary_text
            })
            .block(Block::default().title("Today").borders(Borders::ALL));
            f.render_widget(summary_block, chunks[1]);

            let weekly_total_secs: i64 = weekly_summary.iter().map(|(_, s)| s).sum();
            let mut weekly_lines = weekly_summary
                .iter()
                .map(|(name, total_secs)| {
                    let hours = total_secs / 3600;
                    let mins = (total_secs % 3600) / 60;
                    format!("{:<20} {}h {}m", name, hours, mins)
                })
                .collect::<Vec<_>>();
            if !weekly_lines.is_empty() {
                let total_h = weekly_total_secs / 3600;
                let total_m = (weekly_total_secs % 3600) / 60;
                weekly_lines.push(format!("\n{}h {:02}m of deep work this week!", total_h, total_m));
            }
            let weekly_text = weekly_lines.join("\n");

            let weekly_block = Paragraph::new(if weekly_text.is_empty() {
                String::from("No sessions this week")
            } else {
                weekly_text
            })
            .block(Block::default().title("This Week").borders(Borders::ALL));
            f.render_widget(weekly_block, chunks[2]);

            let input_widget = Paragraph::new(format!("> {}", input))
                .block(Block::default().title("Command").borders(Borders::ALL));
            f.render_widget(input_widget, chunks[3]);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            // checks every 250 ms for an input
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        let command = input.trim().to_string();
                        input.clear();

                        if command.starts_with("start ") {
                            let task = command.strip_prefix("start ").unwrap_or("").trim();
                            tracker::session::start_session(pool, task).await;
                        } else if command == "stop" {
                            tracker::session::stop_session(pool).await;
                        }

                        active_session = tracker::session::get_active_session(pool).await;
                        daily_summary = tracker::session::get_daily_summary(pool).await;
                        weekly_summary = tracker::session::get_weekly_summary(pool).await;
                    }

                    _ => {} // catch all for random shit
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
