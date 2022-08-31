use std::collections::HashMap;

use serde::Deserialize;

/// https://openweathermap.org/api/geocoding-api
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct GeocodeResponse {
    name: String,
    local_names: HashMap<String, String>,
    lat: f64,
    lon: f64,
    country: String,
}
