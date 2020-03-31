#![warn(clippy::all, clippy::pedantic)]

use std::path::PathBuf;

use anyhow::Result;

mod bootstrap;
pub mod config;
mod constants;

pub use self::bootstrap::*;
use lib_service::{TimeClient, TimeServer, WeatherClient, WeatherServer};

pub fn init() -> Result<()> {
    // Load configuration
    let config = config::Config::from_file(&PathBuf::from(constants::CONFIG_PATH))?;

    // Initialize logger
    init_logger(&config)?;

    // Initialize services
    let time_client = TimeClient::new(TimeServer::spawn(create_time_service(&config)));
    let weather_client = WeatherClient::new(WeatherServer::spawn(create_weather_service(&config)));

    // Start UI
    let ui_config = lib_ui_framework::Config {
        width: config.display.width,
        height: config.display.height,
    };
    lib_ui::start(
        lib_ui_framework::System::new(ui_config),
        lib_ui::State {
            time_client,
            weather_client,
        },
    );

    Ok(())
}
