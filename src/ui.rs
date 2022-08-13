use chrono::offset::Local;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::handlers::{app::App, config::CompleteConfig};

#[allow(unused_variables)]
pub fn draw_ui<T: Backend>(frame: &mut Frame<T>, app: &mut App, config: &CompleteConfig) {
    let vertical_chunk_constraints = vec![Constraint::Min(1)];

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_chunk_constraints.as_ref())
        .split(frame.size());

    let paragraph = Paragraph::new("Something").block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("[ {} ]", Local::now().format("%c"))),
    );

    frame.render_widget(paragraph, vertical_chunks[0]);
}
