#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::unused_self,
    clippy::too_many_lines,
    clippy::future_not_send
)]

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
