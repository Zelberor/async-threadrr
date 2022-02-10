use std::sync::Arc;

use super::task_spawn::Task;
use flume::Receiver;

pub trait Run {
    fn run(&self);
}

pub struct Runner {
    receiver: Receiver<Arc<dyn Task>>,
}

impl Runner {
    pub fn new(receiver: &Receiver<Arc<dyn Task>>) -> Runner {
        Runner {
            receiver: receiver.clone(),
        }
    }
}

impl Run for Runner {
    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            task.poll();
        }
    }
}
