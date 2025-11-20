use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(1),      // Main content area
                Constraint::Length(3),   // Input box area
            ]
            .as_ref(),
        )
        .split(f.size());

    // Create a Paragraph to display the output
    let output_paragraph = Paragraph::new(app.output.as_str())
        .block(Block::default().title("Output").borders(Borders::ALL));
    
    // Render the output Paragraph in the top chunk
    f.render_widget(output_paragraph, chunks[0]);

    // Create a Paragraph for the command input
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Command"));
    
    // Render the input Paragraph
    f.render_widget(input, chunks[1]);

    // Set the cursor to be visible
    f.set_cursor(
        // The X position of the cursor
        chunks[1].x + app.input.len() as u16 + 1,
        // The Y position of the cursor
        chunks[1].y + 1,
    );
}
