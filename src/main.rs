mod api;
mod handlers;
mod terminal;
mod ui;
mod utils;

use color_eyre::eyre::{Result, WrapErr};

use crate::handlers::{app::App, config::CompleteConfig};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let config = CompleteConfig::new()
        .wrap_err("Fatal configuration error.")
        .unwrap();

    let app = App::new(config.clone()).await;

    terminal::ui_driver(config, app).await;

    std::process::exit(0)
}
