use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
/// The overall configuration for the operation
pub struct Config {
    pub debug: bool,
    /// The language the application should render in
    pub language: Language,
    /// The chosen air quality service
    pub air_service: AirServiceChoice,
    /// The chosen time service
    pub time_service: TimeServiceChoice,
    /// The chosen weather service
    pub weather_service: WeatherServiceChoice,
}

#[derive(Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "en")]
    English,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum AirServiceChoice {
    Dummy,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TimeServiceChoice {
    Chrono,
    Dummy,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// The possibly weather service options
pub enum WeatherServiceChoice {
    DarkSky,
    Dummy,
    OpenWeatherMap,
}
