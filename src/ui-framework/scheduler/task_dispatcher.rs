use crate::{Task, TaskTx};
use anyhow::Result;

pub struct TaskDispatcher {
    tx: TaskTx,
}

impl TaskDispatcher {
    pub(crate) fn new(tx: TaskTx) -> Self {
        Self { tx }
    }

    pub fn enqueue(&self, task: Task) -> Result<()> {
        self.tx.send(task)?;
        Ok(())
    }
}
