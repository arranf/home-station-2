use anyhow::Result;
use std::sync::mpsc::Sender;

use lib_service_common::{Weather, WeatherForecast};

#[derive(Debug)]
pub enum WeatherRequest {
    /// A request to get the current `Weather`
    GetWeather { tx: Sender<Result<Weather>> },
    /// A request to get a forward looking `WeatherForecast`
    GetWeatherForecast { tx: Sender<Result<WeatherForecast>> },
}
