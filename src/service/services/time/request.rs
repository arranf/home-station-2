use std::sync::mpsc::Sender;

use lib_service_common::Time;

#[derive(Debug)]
pub enum TimeRequest {
    /// A request to get the current `Time`
    GetTime { tx: Sender<Time> },
}
