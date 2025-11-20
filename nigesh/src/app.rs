use crossterm::event::KeyCode;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::AsyncWriteExt;

pub struct App {
    pub input: String,
    pub messages: Vec<String>,
    pub tick_count: u64,
    pub should_quit: bool,
    pub read_half: Option<OwnedReadHalf>,
    pub write_half: Option<OwnedWriteHalf>,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            tick_count: 0,
            should_quit: false,
            read_half: None,
            write_half: None,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

    pub async fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                if self.input.trim() == "/quit" {
                    self.should_quit = true;
                    return ; 
                } 
                if let Some(write_half) = &mut self.write_half{
                    let mut msg = self.input.drain(..).collect::<String>(); 
                    msg.push('\n'); 

                    if write_half.write_all(msg.as_bytes()).await.is_ok(){
                    } else {
                        self.messages.push("Failed to send message.".to_string());
                    }
                }


                else {
                    self.messages.push(self.input.drain(..).collect());
                }
            }
            _ => {}
        }
    }
}
