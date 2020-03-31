use log::debug;

use std::fmt::Debug;
use std::sync::mpsc::{channel, Sender};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub use self::{command::*, token::*};

mod command;
mod token;

#[derive(Debug, Clone)]
pub struct ServicePoller<Client: Clone + Debug + Send> {
    /// The `Client` that should be interacted with on an interval.
    client: Client,
    /// How frequently the `Client` should be interacted with
    interval: Option<Duration>,
}

impl<Client: Clone + Debug + Send + 'static> ServicePoller<Client> {
    /// Creates a `ServicePoller`.
    pub fn new(client: &Client) -> Self {
        Self {
            // @todo: What are the implications of cloning this?
            client: client.clone(),
            interval: None,
        }
    }

    /// Sets how frequently the poller should run.
    pub fn each(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Sends a function that should be run by the poller on a loop until termination
    /// Returns a `ServicePollerToken`.
    pub fn send<Event: Send + 'static>(
        self,
        tx: &Sender<Event>,
        mut handler: impl (FnMut(&mut Client) -> Event) + Send + 'static,
    ) -> ServicePollerToken {
        // @todo Figure out the implications of this clone.
        let tx = tx.clone();
        self.run(move |client| {
            tx.send(handler(client)).unwrap();
        })
    }

    // Runs the requested function in a loop on a new thread
    fn run(self, mut handler: impl FnMut(&mut Client) + Send + 'static) -> ServicePollerToken {
        let mut client = self.client;
        let interval = self.interval.expect(&format!(
            "Expected an interval to be provided for {:?} to run at.",
            client.clone()
        ));

        let (tx, rx) = channel();

        spawn(move || loop {
            if rx.try_recv() == Ok(ServicePollerCommand::Terminate) {
                debug!("Receiving terminate command");
                return;
            }

            handler(&mut client);
            sleep(interval);
        });

        ServicePollerToken::new(tx)
    }
}
