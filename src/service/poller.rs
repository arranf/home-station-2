use std::sync::mpsc::{channel, Sender};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub use self::{command::*, token::*};

mod command;
mod token;

pub struct ServicePoller<Client> {
    client: Client,
    interval: Option<Duration>,
}

impl<Client: Clone> ServicePoller<Client> {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
            interval: None,
        }
    }
}

impl<Client> ServicePoller<Client> {
    pub fn each(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }
}

impl<Client: Send + 'static> ServicePoller<Client> {
    pub fn run(self, mut handler: impl FnMut(&mut Client) + Send + 'static) -> ServicePollerToken {
        let mut client = self.client;
        let interval = self.interval.unwrap();

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

    pub fn send<Event: Send + 'static>(
        self,
        tx: &Sender<Event>,
        mut handler: impl (FnMut(&mut Client) -> Event) + Send + 'static,
    ) -> ServicePollerToken {
        let tx = tx.clone();

        self.run(move |client| {
            tx.send(handler(client)).unwrap();
        })
    }
}
