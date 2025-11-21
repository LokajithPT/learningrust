use ratatui::{
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let main_block = Block::default().borders(Borders::ALL).title("Todo List");
    let placeholder = Paragraph::new("Nothing to see here yet.").block(main_block);
    f.render_widget(placeholder, f.area());
}