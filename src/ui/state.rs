use lib_service::{TimeClient, WeatherClient};

#[derive(Debug)]
pub struct State {
    pub time_client: TimeClient,
    pub weather_client: WeatherClient,
}
