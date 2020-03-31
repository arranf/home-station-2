use anyhow::{Context, Result};

use std::sync::mpsc::{channel, Sender};

use lib_service_common::{Weather, WeatherForecast};

use crate::WeatherRequest;

#[derive(Clone)]
pub struct WeatherClient {
    // `WeatherServer` has the `Receiver`
    tx: Sender<WeatherRequest>,
}

impl WeatherClient {
    pub fn new(tx: Sender<WeatherRequest>) -> Self {
        Self { tx }
    }

    pub fn get_weather(&self) -> Result<Weather> {
        // We create a channel so that we can communicate with the `WeatherServer`.
        let (tx, rx) = channel();

        // Sends a `WeatherRequest` to sender half of the channel stored in `WeatherClient`, which is held by `WeatherServer`.
        // We pass in the sender half of the channel we create in this function so they can send the result back to us.
        self.tx
            .send(WeatherRequest::GetWeather { tx })
            // @todo Figure out error handling here or decide if panicking is the correct thing to do here.
            .expect("Error sending Weather Request");

        Ok(rx
            .recv()?
            .with_context(|| "Error receiving from WeatherServer")?)
    }

    pub fn get_weather_forecast(&self) -> Result<WeatherForecast> {
        // We create a channel so that we can communicate with the `WeatherServer`.
        let (tx, rx) = channel();

        self.tx
            .send(WeatherRequest::GetWeatherForecast { tx })
            // @todo Figure out error handling here or decide if panicking is the correct thing to do here.
            .expect("Error sending GetWeatherForecast");

        rx.recv()
            .with_context(|| "Error receiving from WeatherServer")?
    }
}
