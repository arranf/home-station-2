use std::sync::mpsc::{Receiver, Sender};

pub type ServicePollerCommandTx = Sender<ServicePollerCommand>;
pub type ServicePollerCommandRx = Receiver<ServicePollerCommand>;

#[derive(Debug, Eq, PartialEq)]
/// Commands that can be sent to the service poller
pub enum ServicePollerCommand {
    Terminate,
}
