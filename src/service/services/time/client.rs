use anyhow::{Context, Result};

use std::sync::mpsc::{channel, Sender};

use lib_service_common::Time;

use crate::TimeRequest;

#[derive(Clone)]
pub struct TimeClient {
    // `TimeServer` has the receiver half of this channel.
    tx: Sender<TimeRequest>,
}

impl TimeClient {
    /// Creates a new `TimeClient` from a `Sender`.
    pub fn new(tx: Sender<TimeRequest>) -> Self {
        Self { tx }
    }

    /// Gets the current `Time`
    pub fn get_time(&self) -> Result<Time> {
        // We create a channel to communicate with the `TimeServer`.
        let (tx, rx) = channel();

        // Sends a `TimeRequest` to the sender half of the channel that's stored in the `TimeClient`, which is held by `TimeServer`.
        // The request contains the sender half of the channel created in this function so that this function can receive communication back.
        self.tx
            .send(TimeRequest::GetTime { tx })
            // @todo Figure out how to make this error handling better or confirm that panicking here is the correct thing to do.
            .expect("Error sending time request");

        Ok(rx
            .recv()
            .with_context(|| "Failed receiving time in TimeClient")?)
    }
}
