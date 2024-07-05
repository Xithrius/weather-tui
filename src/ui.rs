use std::string::{String, ToString};

// use chrono::{DateTime, Local, NaiveDateTime, Utc};
use chrono::Local;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, Row, Table},
};

use crate::{
    handlers::{app::App, config::CompleteConfig},
    utils::text::vector_column_max,
};

pub fn draw_ui(frame: &mut Frame, app: &App, config: &CompleteConfig) {
    let v_constraints = vec![Constraint::Min(1)];

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(config.frontend.margin)
        .constraints(v_constraints)
        .split(frame.size());

    let mut interval_data = if let Some(data) = app.weather_data.clone() {
        data.list
            .iter()
            .map(|item| {
                vec![
                    // DateTime::<Utc>::from_naive_utc_and_offset(
                    //     DateTime::from_naive_utc_and_offset(item.dt.try_into().unwrap(), 0).unwrap(),
                    //     Utc,
                    // )
                    // .to_string(),
                    item.weather[0].description.clone(),
                    item.main.temp.to_string(),
                    item.main.feels_like.to_string(),
                ]
            })
            .collect::<Vec<Vec<String>>>()
    } else {
        vec![]
    };

    let table_headers = ["Date/time", "Description", "Temperature", "Feels like"]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    interval_data.push(table_headers.clone());

    let table_widths = vector_column_max(&interval_data)
        .map(Constraint::Min)
        .collect::<Vec<Constraint>>();

    interval_data.pop();

    let table = Table::new(
        interval_data
            .iter()
            .map(|i| Row::new(i.iter().map(String::as_str))),
        &[Constraint::Percentage(100)],
    )
    .block(Block::default().borders(Borders::ALL).title(format!(
        "[ {} ] [ Area: {} ] [ Units: {} ]",
        Local::now().format("%c"),
        config.weather.area,
        config.weather.units
    )))
    .widths(&table_widths)
    .column_spacing(2)
    .header(Row::new(table_headers).style(Style::default().add_modifier(Modifier::BOLD)));

    frame.render_widget(table, v_chunks[0]);
}
