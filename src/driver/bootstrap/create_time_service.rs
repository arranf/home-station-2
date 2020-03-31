use lib_service_common::TimeService;
use lib_service_providers::{chrono, dummy};

use crate::config::{app::TimeServiceChoice, Config};

#[must_use]
///  Creates the correct `TimeService` given a `Config`.
pub fn create_time_service(config: &Config) -> Box<dyn TimeService> {
    match config.app.time_service {
        TimeServiceChoice::Chrono => Box::new(chrono::Service::new()),

        TimeServiceChoice::Dummy => Box::new(dummy::Provider::new()),
    }
}
