use color_eyre::eyre::{Error, Result, WrapErr};

mod api;
mod handlers;
mod terminal;
mod ui;
mod utils;

use crate::{
    handlers::{app::App, config::CompleteConfig},
    terminal::ui_driver,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    color_eyre::install().unwrap();

    let config = CompleteConfig::new().wrap_err("Unable to read configuration file.")?;

    let app = App::new(config.clone()).await;

    ui_driver(config, app).await;

    Ok(())
}
