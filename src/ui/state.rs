use lib_service::{TimeClient, WeatherClient};

#[derive(Debug)]
pub struct State {
    pub time: TimeClient,
    pub weather: WeatherClient,
}
