use color_eyre::eyre::{bail, Error, Result};

use crate::{api::responses::GeocodeResponse, handlers::config::WeatherConfig};

mod responses;

#[derive(Debug)]
pub struct OpenWeatherMap {
    api_key: String,
    area_data: GeocodeResponse,
}

impl OpenWeatherMap {
    pub async fn new(config: WeatherConfig) -> Result<Self, Error> {
        let url = format!(
            "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            config.area, config.api_key
        );

        let info = reqwest::get(url)
            .await
            .unwrap()
            .json::<Vec<GeocodeResponse>>()
            .await
            .unwrap();

        if info.is_empty() {
            bail!("Geocode response was empty, could not locate area.");
        }

        Ok(Self {
            api_key: config.api_key,
            area_data: info[0].clone(),
        })
    }

    pub async fn get_temp(&self) -> String {
        todo!()
    }
}
