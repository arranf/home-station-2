use lib_service_common::WeatherService;
use lib_service_providers::{dark_sky, dummy, open_weather_map};

use crate::config::{app::WeatherServiceChoice, Config};

#[must_use]
/// Creates the correct `WeatherService` given `Config`.
pub fn create_weather_service(config: &Config) -> Box<dyn WeatherService> {
    match config.app.weather_service {
        WeatherServiceChoice::DarkSky => {
            let config = config
                .services
                .dark_sky.as_ref().expect("DarkSky has not been configured - please create the [services.dark-sky] section.")
                .clone();

            Box::new(dark_sky::Service::new(config))
        }

        WeatherServiceChoice::Dummy => Box::new(dummy::Provider::new()),

        WeatherServiceChoice::OpenWeatherMap => {
            let config = config
                .services
                .open_weather_map.as_ref().expect("OpenWeatherMap has not been configured - please create the [services.open-weather-map] section.")
                .clone();

            Box::new(open_weather_map::Provider::new(config))
        }
    }
}
