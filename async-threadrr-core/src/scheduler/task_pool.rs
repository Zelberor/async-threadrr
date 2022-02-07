use std::sync::Arc;

use flume::{Receiver, Sender};

use super::task_spawn::Task;

struct TaskPool {
    receiver: Receiver<Arc<dyn Task>>,
    sender: Sender<Arc<dyn Task>>,
}

impl TaskPool {
    fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self { sender, receiver }
    }

    fn sender(&self) -> &Sender<Arc<dyn Task>> {
        &self.sender
    }

    fn receiver(&self) -> &Receiver<Arc<dyn Task>> {
        &self.receiver
    }
}
