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
    /// Everything to do with the weather
    pub weather: WeatherConfig,
    /// Internal functionality
    pub terminal: TerminalConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct WeatherConfig {
    /// API key for receiving the weather in a location
    pub api_key: String,
    /// The area (city,state_code) to scan
    pub area: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct TerminalConfig {
    /// The delay in milliseconds between terminal updates
    pub tick_delay: u64,
    /// What file to log to, if any
    pub log_file: String,
}

#[allow(dead_code)]
pub struct FrontendConfig {
    /// The date/time format of any timestamp
    pub date_time_format: String,
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        let path_str = config_path("config.toml");

        let p = Path::new(&path_str);

        if !p.exists() {
            create_dir_all(p.parent().unwrap()).unwrap();

            let default_toml_string = toml::to_string(&CompleteConfig::default()).unwrap();
            let mut file = File::create(path_str.clone()).unwrap();
            file.write_all(default_toml_string.as_bytes()).unwrap();

            bail!("Configuration was generated at {path_str}, please fill it out with necessary information.")
        } else if let Ok(config_contents) = read_to_string(&p) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            // merge_args_into_config(&mut config, cli);

            Ok(config)
        } else {
            bail!(
                "Configuration could not be read correctly. See the following link for the example config: {}",
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}
