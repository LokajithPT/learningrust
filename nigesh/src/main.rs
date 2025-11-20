use crate::app::App;
use crate::event::{Event, EventHandler};
use crate::tui::Tui;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{error::Error, io, time::Duration};
use tokio::net::TcpStream; 
use tokio::io::AsyncReadExt; 



mod app;
mod event;
mod tui;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal, EventHandler::new(Duration::from_millis(250)));
    tui.init()?;


    //connecting to the server 
    //
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(stream) => {
            let (read_half, write_half) = stream.into_split();
            app.read_half = Some(read_half);
            app.write_half = Some(write_half);
            app.messages.push("Connected to server.".to_string()); 
        }
        Err(_) => {
            app.messages.push("Failed to connect to server.".to_string());
            
        }
    }

    if let Some(mut read_half) = app.read_half.take() {
        let event_sender = tui.events.sender(); 

        tokio::spawn(async move {
            let mut buffer = vec![0;1024];
            loop{
                match read_half.read(&mut buffer).await{
                    Ok(0) => {
                        // Connection closed
                        break;
                    }
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
                        if event_sender.send(Event::ServerMessage(msg)).await.is_err(){
                            break; 
                        }
                    }
                    Err(_) => {
                        // Error occurred
                        break;
                    }
                }
            }
        }); 


    }


    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await {
            Some(Event::Tick) => app.tick(),
            Some(Event::Key(key_event)) => app.handle_key_event(key_event).await,
            Some(Event::ServerMessage(msg)) => {
                app.messages.push(msg); 
            }

            None => break,
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
