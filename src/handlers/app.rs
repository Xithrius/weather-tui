use std::collections::VecDeque;

use rustyline::line_buffer::LineBuffer;

pub enum State {
    Normal,
}

pub struct App {
    /// 2D vector containing weather data
    pub weather_data: VecDeque<Vec<String>>,
    /// Which window is within the focused layer
    pub state: State,
    /// Text input for the user
    pub line_buffer: LineBuffer,
    /// Scrolling offset for windows.
    pub scroll_offset: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            weather_data: VecDeque::new(),
            state: State::Normal,
            line_buffer: LineBuffer::with_capacity(4096),
            scroll_offset: 0,
        }
    }
}
