use crate::app::App;
use crate::event::EventHandler;
use ratatui::{backend::Backend, Terminal};
use std::io;

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn init(&mut self) -> io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> io::Result<()> {
        self.terminal.draw(|frame| crate::ui::ui(frame, app))?;
        Ok(())
    }

    pub fn exit(&mut self) -> io::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
