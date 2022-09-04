#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;
use tui::widgets::Row;

trait ToRow {
    fn to_row(&self) -> Row;
}

/// Response after requesting information of an area
/// <https://openweathermap.org/api/geocoding-api/>
#[derive(Deserialize, Debug, Clone)]
pub struct GeocodeResponse {
    name: String,
    local_names: HashMap<String, String>,
    pub lat: f64,
    pub lon: f64,
    country: String,
}

/// Format for the forecast of 5 days, at 3 hour intervals
/// <https://openweathermap.org/forecast5/>
#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5Response {
    /// Internal parameter
    cod: String,
    /// Internal parameter
    message: usize,
    /// A number of timestamps returned in the API response
    cnt: usize,
    /// List of 3 hour interval forecasts
    pub list: Vec<Forecast5ListItem>,
    /// Information of the city with the requested location
    city: Forecast5City,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItem {
    /// Time of data forecasted, unix, UTC
    pub dt: usize,
    ///
    pub main: Forecast5ListItemMain,
    ///
    pub weather: Vec<Forecast5ListItemWeather>,
    ///
    pub clouds: Forecast5ListItemClouds,
    ///
    pub wind: Forecast5ListItemWind,
    /// Average visibility, metres with a maximum of 10km
    visibility: usize,
    /// Probability of precipitation, %
    pop: f64,
    ///
    rain: Option<Forecast5ListItemRain>,
    ///
    sys: Forecast5ListItemSys,
    /// Time of data forecasted, ISO, UTC
    dt_txt: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItemMain {
    /// Temperature
    temp: f64,
    /// This temperature parameter accounts for the human perception of weather
    feels_like: f64,
    /// Minimum temperature at the moment of calculation
    temp_min: f64,
    /// Maximum temperature at the moment of calculation
    temp_max: f64,
    /// Atmospheric pressure on the sea level by default, hPa
    pressure: usize,
    /// Atmospheric pressure on the sea level, hPa
    sea_level: usize,
    /// Atmospheric pressure on the ground level
    grnd_level: usize,
    /// Humidity, %
    humidity: usize,
    /// Internal parameter
    temp_kf: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItemWeather {
    /// Weather condition id
    id: usize,
    /// Group of weather parameters (Rain, Snow, Extreme etc)
    main: String,
    /// Weather condition within the group
    pub description: String,
    /// Weather icon id
    icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItemClouds {
    /// Cloudiness, %
    all: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItemWind {
    /// Wind speed
    speed: f64,
    /// Wind direction, degrees (meteorological)
    deg: f64,
    /// Wind gust
    gust: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItemRain {
    /// Rain volume for last 3 hours, mm
    _3h: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5ListItemSys {
    /// Part of the day (n - night, d - day)
    pod: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5City {
    /// City ID
    id: usize,
    /// City name
    name: String,
    ///
    coord: Forecast5CityCoord,
    /// Country code (GB, JP etc)
    country: String,
    /// Population size
    population: usize,
    /// Shift in seconds from UTC
    timezone: isize,
    /// Sunrise time, Unix, UTC
    sunrise: usize,
    /// Sunset time, Unix, UTC
    sunset: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast5CityCoord {
    /// City geo location, latitude
    lat: f64,
    /// City geo location, longitude
    lon: f64,
}
