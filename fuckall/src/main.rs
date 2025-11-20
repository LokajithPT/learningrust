use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io, process::Command};
use sysinfo::System;

mod app;
use app::App;
mod ui;
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Run the main loop
    let res = run_app(&mut terminal);

    // 3. Restore the terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    let mut sys = System::new_all();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        let command_str = app.input.drain(..).collect::<String>();
                        let mut parts = command_str.trim().split_whitespace();
                        let command = parts.next().unwrap_or("");
                        let args = parts;

                        match command {
                            "cpu" | "mem" | "clear" => {
                                // Handle built-in commands
                                match command {
                                    "cpu" => {
                                        sys.refresh_cpu();
                                        app.output = format!("CPU Cores: {}", sys.cpus().len());
                                    }
                                    "mem" => {
                                        sys.refresh_memory();
                                        app.output = format!(
                                            "Total Memory: {} KB\nUsed Memory: {} KB",
                                            sys.total_memory(),
                                            sys.used_memory()
                                        );
                                    }
                                    "clear" => {
                                        app.output.clear();
                                    }
                                    _ => {} // Unreachable
                                }
                            }
                            "cd" => {
                                app.output = "Error: 'cd' is a shell builtin and cannot be executed.".to_string();
                            }
                            "" => { // Handle empty input
                                // Do nothing
                            }
                            _ => { // Handle external command
                                let output = Command::new(command).args(args).output();
                                match output {
                                    Ok(output) => {
                                        let stdout = String::from_utf8_lossy(&output.stdout);
                                        let stderr = String::from_utf8_lossy(&output.stderr);
                                        app.output = format!("{}\n{}", stdout, stderr);
                                    }
                                    Err(e) => {
                                        app.output = format!("Failed to execute command '{}': {}", command, e);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

