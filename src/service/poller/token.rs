use crate::{ServicePollerCommand, ServicePollerCommandTx};

pub struct ServicePollerToken {
    tx: ServicePollerCommandTx,
}

impl ServicePollerToken {
    pub(crate) fn new(tx: ServicePollerCommandTx) -> Self {
        Self { tx }
    }
}

impl Drop for ServicePollerToken {
    // When dropped the poller should terminate
    fn drop(&mut self) {
        let _ = self.tx.send(ServicePollerCommand::Terminate);
    }
}
