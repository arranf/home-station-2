use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

pub mod app;
pub mod display;
pub mod services;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub app: app::Config,
    pub display: display::Config,
    pub services: services::Config,
}

impl Config {
    /// Loads home-station-2's configuration from a specified file.
    #[must_use]
    pub fn from_file(path: &Path) -> Result<Self> {
        let config = fs::read_to_string(path).with_context(|| {
            format!(
                "Failed to read configuration file from {}",
                &path.to_string_lossy()
            )
        })?;

        Ok(toml::from_str(&config)?)
    }
}
