use lib_service_providers::{airly, dark_sky, open_weather_map};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The configuration for the (as of yet unfinished) [Airly](https://airly.eu/en/) integration, see `airly::Config`.
    pub airly: Option<airly::Config>,
    /// The configuration for the [Dark Sky](https://darksky.net/dev) integration, see `dark_sky::Config`.
    pub dark_sky: Option<dark_sky::Config>,
    /// The configuration for the [Open Weather Map](https://openweathermap.org/) integration, see `open_weather_map::Config`.
    pub open_weather_map: Option<open_weather_map::Config>,
}
