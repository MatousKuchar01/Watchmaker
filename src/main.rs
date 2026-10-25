mod app;
mod ui;
mod event;
mod ascii;

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
use app::{App, Screen};

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
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match app.screen {
                    // Navigation in main menu
                    Screen::MainMenu => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.quit();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.menu_next();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.menu_previous();
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            app.menu_select();
                        }
                        _ => {}
                    },
                    Screen::Workbench => match key.code {
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
                    Screen::Catalog => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app.screen = Screen::MainMenu
                        },
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.catalog_next()
                        },
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.catalog_previous()
                        },
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            app.catalog_select_current()
                        },
                        _ => {}
                    },
                }
            },
            _ => {}
        }
    }


    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}