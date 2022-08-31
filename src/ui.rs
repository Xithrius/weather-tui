use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    handlers::{
        app::{App, State},
        config::CompleteConfig,
    },
    utils::text::get_cursor_position,
};

pub fn draw_ui<T: Backend>(frame: &mut Frame<T>, app: &mut App, config: &CompleteConfig) {
    let v_constraints = match app.state {
        State::Insert => vec![Constraint::Min(1), Constraint::Length(3)],
        _ => vec![Constraint::Min(1)],
    };

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(config.frontend.margin)
        .constraints(v_constraints.as_ref())
        .split(frame.size());

    let table = Paragraph::new("Some text").block(Block::default().borders(Borders::ALL));

    frame.render_widget(table, v_chunks[0]);

    if let State::Insert = app.state {
        let input_buffer = &app.input_buffer;

        let cursor_pos = get_cursor_position(input_buffer);
        let input_rect = v_chunks[v_constraints.len() - 1];

        frame.set_cursor(
            (input_rect.x + cursor_pos as u16 + 1)
                .min(input_rect.x + input_rect.width.saturating_sub(2)),
            input_rect.y + 1,
        );

        let paragraph = Paragraph::new(Spans::from(vec![Span::raw(input_buffer.as_str())]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("[ Input ]")
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .scroll((
                0,
                ((cursor_pos + 3) as u16).saturating_sub(input_rect.width),
            ));

        frame.render_widget(paragraph, input_rect);
    }
}
