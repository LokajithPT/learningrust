use crate::app::App;
use crate::event::{Event, EventHandler};
use crate::tui::Tui;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{error::Error, io, time::Duration};

mod app;
mod event;
mod tui;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create application state
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal, EventHandler::new(Duration::from_millis(250)));
    tui.init()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.
        match tui.events.next().await {
            Some(Event::Tick) => {} // Tick event, for now does nothing
            Some(Event::Key(key_code)) => {
                // For now, we only handle 'q' to quit.
                if key_code == crossterm::event::KeyCode::Char('q') {
                    app.should_quit = true;
                }
            }
            None => {
                // Event stream closed
                app.should_quit = true;
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
