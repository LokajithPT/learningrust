use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(f.area());

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();
    let messages_list =
        List::new(messages).block(Block::default().borders(Borders::ALL).title(format!("Messages (Ticks : {})", app.tick_count)));
    f.render_widget(messages_list, chunks[0]);

    let input_paragraph = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Input").borders(Borders::ALL));
    f.render_widget(input_paragraph, chunks[1]);

    f.set_cursor_position((
        chunks[1].x + app.input.len() as u16 + 1,
        chunks[1].y + 1,
    ));
}
