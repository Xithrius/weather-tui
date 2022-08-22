use std::collections::HashMap;

use color_eyre::eyre::{bail, Error, Result};
use serde::Deserialize;

use crate::handlers::config::WeatherConfig;

/// https://openweathermap.org/api/geocoding-api
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct GeocodeResponse {
    name: String,
    local_names: HashMap<String, String>,
    lat: f64,
    lon: f64,
    country: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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

    #[allow(dead_code)]
    pub async fn get_temp(&self) -> String {
        todo!()
    }
}
