use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
};
use std::io;
use crate::tracker;


pub async fn run(pool: &sqlx::PgPool) -> io::Result<()> {
    enable_raw_mode()?; // raw mode rather than cooked mode, doesn't require an enter at the end for everything
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // ? stops it from crashing

    let backend = CrosstermBackend::new(stdout); // talks from ratatui to terminal
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut active_session = tracker::session::get_active_session(pool).await;

    // event loop prompting us for actions
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
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

            let status_block = Paragraph::new(status_text)
                .block(Block::default().title("productiviCLI").borders(Borders::ALL));
            f.render_widget(status_block, chunks[0]);

            let input_widget = Paragraph::new(format!("> {}", input))
                .block(Block::default().title("Command").borders(Borders::ALL));
            f.render_widget(input_widget, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? { // checks every 250 ms for an input
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => { input.pop(); },
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
