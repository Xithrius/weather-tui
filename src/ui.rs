use std::string::String;

use chrono::{DateTime, NaiveDateTime, Utc};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::{
    handlers::{
        app::{App, State},
        config::CompleteConfig,
    },
    utils::text::{get_cursor_position, vector_column_max},
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

    // let if let interval_data = app.weather_data;

    let interval_data = if let Some(data) = app.weather_data.clone() {
        data.list
            .iter()
            .map(|item| {
                vec![
                    DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(item.dt as i64, 0),
                        Utc,
                    )
                    .to_string(),
                    item.weather[0].description.clone(),
                ]
            })
            .collect::<Vec<Vec<String>>>()
    } else {
        vec![]
    };

    let table_widths = vector_column_max(&interval_data)
        .into_iter()
        .map(Constraint::Min)
        .collect::<Vec<Constraint>>();

    let table = Table::new(
        interval_data
            .iter()
            .map(|i| Row::new(i.iter().map(String::as_str).collect::<Vec<&str>>()))
            .collect::<Vec<Row>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("[ Weather ]"))
    .widths(&table_widths);

    frame.render_widget(table, v_chunks[0]);

    if app.state == State::Insert {
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
