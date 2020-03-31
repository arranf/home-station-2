use lib_service::{TimeClient, WeatherClient};

/// The state used by the UI for rendering
#[derive(Debug)]
pub struct State {
    pub time_client: TimeClient,
    pub weather_client: WeatherClient,
}
