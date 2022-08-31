use std::{cmp::Eq, hash::Hash};

use color_eyre::eyre::WrapErr;
use rustyline::line_buffer::LineBuffer;

use crate::{CompleteConfig, api::OpenWeatherMap};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum State {
    Normal,
    Insert,
    Help,
}

#[derive(Debug)]
pub struct App {
    /// The current state the application is in
    pub state: State,
    /// An input box for the user
    pub input_buffer: LineBuffer,
    /// Interactions with the weather API.
    pub api: OpenWeatherMap,
}

impl App {
    pub async fn new(config: CompleteConfig) -> Self {
        let weather_api = OpenWeatherMap::new(config.weather)
            .await
            .wrap_err("Could not configure weather API.")
            .unwrap();

        Self {
            state: State::Normal,
            input_buffer: LineBuffer::with_capacity(4096),
            api: weather_api,
        }
    }
}
