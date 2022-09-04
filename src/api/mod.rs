use color_eyre::eyre::{bail, Error, Result};

use crate::{
    api::responses::{Forecast5Response, GeocodeResponse},
    handlers::config::WeatherConfig,
};

pub mod responses;

#[derive(Debug)]
pub struct OpenWeatherMap {
    api_key: String,
    area: GeocodeResponse,
}

impl OpenWeatherMap {
    pub async fn new(config: WeatherConfig) -> Result<Self, Error> {
        let url = format!(
            "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            config.area, config.api_key
        );

        let info: Vec<GeocodeResponse> = reqwest::get(url).await.unwrap().json().await.unwrap();

        if info.is_empty() {
            bail!("Geocode response was empty, could not locate area.");
        }

        Ok(Self {
            api_key: config.api_key,
            area: info[0].clone(),
        })
    }

    pub async fn get_5_day_forecast(&self) -> Forecast5Response {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}",
            self.area.lat, self.area.lon, self.api_key
        );

        let response: Forecast5Response = reqwest::get(url).await.unwrap().json().await.unwrap();

        response
    }
}
