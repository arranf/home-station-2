use log::trace;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use lib_service_common::TimeService;

use crate::TimeRequest;

pub struct TimeServer {
    service: Box<dyn TimeService>,
    // `TimeClient` has the `Sender` half of this channel.
    rx: Receiver<TimeRequest>,
}

impl TimeServer {
    /// Starts a `TimeServer` in a separate thread and returns a `Sender` to send requests to it.
    pub fn spawn(service: Box<dyn TimeService>) -> Sender<TimeRequest> {
        let (tx, rx) = channel();

        thread::spawn(move || Self::new(service, rx).start());

        tx
    }

    fn new(service: Box<dyn TimeService>, rx: Receiver<TimeRequest>) -> Self {
        Self { service, rx }
    }

    fn start(mut self) {
        for request in self.rx.iter() {
            trace!("Processing request: {:?}", request);

            match request {
                TimeRequest::GetTime { tx } => {
                    tx.send(self.service.current())
                        .expect("Failed to send current time to Time Client");
                }
            }
        }
    }
}

impl Drop for TimeServer {
    fn drop(&mut self) {
        trace!("Terminating Time Server");
    }
}
