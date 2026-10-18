mod app;
mod ui;
mod event;

use crossterm::{
    event::KeyEventKind,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        disable_raw_mode,
        enable_raw_mode},
    ExecutableCommand,
};
use event::{Event, EventHandler};
use ratatui::prelude::*;
use std::io::{self, stdout};
use crossterm::event::KeyCode;
use app::App;

/// Main game runner
fn main() -> io::Result<()> {
    // Inicialization of terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout()))?
    ;

    let mut app = App::new();

    // Main game loop
    while !app.should_quit {
        // 1. ->> Draw UI
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        // 2. ->> Event control
        match EventHandler::next()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.quit();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.select_next_tool();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.select_previous_tool();
                }
                KeyCode::Enter => {
                    app.use_selected_tool();
                }
                _ => {}
            },
            _ => {}
        }
    }


    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}