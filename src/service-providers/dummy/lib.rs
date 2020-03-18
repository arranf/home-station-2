#[macro_use]
extern crate log;

use chrono::Weekday;

use lib_service_common::{
    Time, TimeService, Weather, WeatherForecast, WeatherIcon, WeatherService,
};

pub struct Provider;

impl Provider {
    pub fn new() -> Self {
        info!("Initializing dummy");

        Self
    }
}

impl TimeService for Provider {
    fn current(&mut self) -> Time {
        Time {
            hour: 13,
            minute: 37,
            second: 0,
            weekday: Weekday::Wed,
            day: 21,
            month: 9,
            year: 1994,
        }
    }
}

impl WeatherService for Provider {
    fn current(&mut self) -> Weather {
        Weather {
            temperature: Some(20.0),
            pressure: Some(950),
            humidity: Some(55),
            wind_speed: Some(4.0),
            icon: Some(WeatherIcon::SunWithCloud),
        }
    }

    fn forecast(&mut self) -> Option<WeatherForecast> {
        None
    }
}
