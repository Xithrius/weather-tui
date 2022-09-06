use color_eyre::eyre::WrapErr;

use crate::{
    api::{responses::Forecast5Response, OpenWeatherMap},
    CompleteConfig,
};

#[derive(Debug)]
pub struct App {
    /// Interactions with the weather API
    pub api: OpenWeatherMap,
    /// Currently saved weather data
    pub weather_data: Option<Forecast5Response>,
}

impl App {
    pub async fn new(config: CompleteConfig) -> Self {
        let weather_api = OpenWeatherMap::new(config.weather)
            .await
            .wrap_err("Could not configure weather API.")
            .unwrap();

        Self {
            api: weather_api,
            weather_data: None,
        }
    }
}
