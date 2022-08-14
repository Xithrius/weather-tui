use std::collections::VecDeque;

use color_eyre::eyre::WrapErr;
use rustyline::line_buffer::LineBuffer;

use crate::api::OpenWeatherMap;

use super::config::CompleteConfig;

pub enum State {
    Normal,
}

pub struct App {
    /// 2D vector containing weather data
    pub data: VecDeque<Vec<String>>,
    /// Which window is within the focused layer
    pub state: State,
    /// Internal OpenWeatherMap handler
    pub weather: OpenWeatherMap,
    /// Text input for the user
    pub line_buffer: LineBuffer,
    /// Scrolling offset for windows.
    pub scroll_offset: usize,
}

impl App {
    pub async fn new(config: CompleteConfig) -> Self {
        let weather_config = OpenWeatherMap::new(config.weather)
            .await
            .wrap_err("Unable to start application")
            .unwrap();

        Self {
            data: VecDeque::new(),
            state: State::Normal,
            weather: weather_config,
            line_buffer: LineBuffer::with_capacity(4096),
            scroll_offset: 0,
        }
    }
}
