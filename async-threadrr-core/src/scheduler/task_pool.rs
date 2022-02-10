use std::sync::Arc;

use flume::{Receiver, Sender};

use super::task_spawn::Task;

pub struct TaskPool
where
    Self: Send + Sync,
{
    receiver: Receiver<Arc<dyn Task>>,
    sender: Sender<Arc<dyn Task>>,
}

impl TaskPool {
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self { sender, receiver }
    }

    pub fn sender(&self) -> &Sender<Arc<dyn Task>> {
        &self.sender
    }

    pub fn receiver(&self) -> &Receiver<Arc<dyn Task>> {
        &self.receiver
    }
}
