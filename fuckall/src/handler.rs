use crate::app::App;
use crossterm::event::KeyCode;
use std::process::Command;
use sysinfo::{System, SystemExt};

pub fn handle_key_event(key: KeyCode, app: &mut App, sys: &mut System>) {
    match key {
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
                    app.output =
                        "Error: 'cd' is a shell builtin and cannot be executed.".to_string();
                }
                "" => { // Handle empty input
                    // Do nothing
                }
                _ => {
                    // Handle external command
                    let output = Command::new(command).args(args).output();
                    match output {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            app.output = format!("{}\n{}", stdout, stderr);
                        }
                        Err(e) => {
                            app.output =
                                format!("Failed to execute command '{}': {}", command, e);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
