use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::io;
use std::time::Duration;

/// Events that can happen in game
pub enum Event {
    /// player pressed a key
    Key(KeyEvent),
    /// time interval passed
    Tick,
}

/// Helper struct for reading events from terminal
pub struct EventHandler;

impl EventHandler {
    /// Reads event from terminal
    /// If player doesn't press any key, it returns tick
    pub fn next() -> Result<Event, io::Error> {
        // did anything happen in 50ms?
        if event::poll(Duration::from_millis(50))? {
            // if key is pressed, wrap it into our Event::Key
            if let CrosstermEvent::Key(key) = event::read()? {
                return Ok(Event::Key(key));
            }
        }

        Ok(Event::Tick)
    }
}