use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::Path,
};

use color_eyre::eyre::{bail, Error, Result};
use serde::{Deserialize, Serialize};

use crate::utils::pathing::config_path;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CompleteConfig {
    /// Internal functionality
    pub terminal: TerminalConfig,
    /// Interacting with the OpenWeatherMap API
    pub weather: WeatherConfig,
    /// What everything looks like to the user
    pub frontend: FrontendConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct TerminalConfig {
    /// How often the terminal will update
    pub tick_delay: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct WeatherConfig {
    /// The key to acquire weather information with
    pub api_key: String,
    /// The area in which the user would like weather data from
    pub area: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FrontendConfig {
    /// The margin around the main window from to the terminal border
    pub margin: u16,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self { tick_delay: 30 }
    }
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        let path_str = config_path("config.toml");

        let p = Path::new(&path_str);

        if !p.exists() {
            create_dir_all(p.parent().unwrap()).unwrap();

            let default_toml_string = toml::to_string(&Self::default()).unwrap();
            let mut file = File::create(path_str.clone()).unwrap();
            file.write_all(default_toml_string.as_bytes()).unwrap();

            bail!("Configuration was generated at {path_str}, please fill it out with necessary information.")
        } else if let Ok(config_contents) = read_to_string(&p) {
            let config: Self = toml::from_str(config_contents.as_str()).unwrap();

            {
                let w = &config.weather;

                if w.api_key.is_empty() || w.area.is_empty() {
                    bail!("Weather key required for this application to run.")
                }
            }

            Ok(config)
        } else {
            bail!(
                "Configuration could not be read correctly. See the following link for the example config: {}",
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}
