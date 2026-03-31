use crossterm::{
    execute, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

pub async fn run(pool: &sqlx::PgPool) -> io::Result<()> {
    enable_raw_mode()?; // raw mode rather than cooked mode, doesn't require an enter at the end for everything
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // ? stops it from crashing

    let backend = CrosstermBackend::new(stdout); // talks from ratatui to terminal
    let mut terminal = Terminal::new(backend)?;


    // event loop prompting us for actions
    loop {
        terminal.draw(|f| {
            // ascii art here i assume
        })?;

        if event::poll(std::time::Duration::from_millis(250))? { // checks every 250 ms for an input
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())

}