use log::trace;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use lib_service_common::WeatherService;

use crate::WeatherRequest;

pub struct WeatherServer {
    service: Box<dyn WeatherService>,
    rx: Receiver<WeatherRequest>,
}

impl WeatherServer {
    fn new(service: Box<dyn WeatherService>, rx: Receiver<WeatherRequest>) -> Self {
        Self { service, rx }
    }

    pub fn spawn(service: Box<dyn WeatherService>) -> Sender<WeatherRequest> {
        let (tx, rx) = channel();

        thread::spawn(move || Self::new(service, rx).start());

        tx
    }

    fn start(mut self) {
        for request in self.rx.iter() {
            trace!("Processing request: {:?}", request);

            match request {
                WeatherRequest::GetWeather { tx } => {
                    tx.send(self.service.current())
                        .expect("Error sending current weather to WeatherClient");
                }

                WeatherRequest::GetWeatherForecast { tx } => {
                    tx.send(self.service.forecast())
                        .expect("Error sending weather forecast to WeatherClient");
                }
            }
        }
    }
}

impl Drop for WeatherServer {
    fn drop(&mut self) {
        trace!("Terminating");
    }
}
