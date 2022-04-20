mod runner;
mod task;

use std::sync::Arc;

use runner::Runner;

use flume::{Receiver, Sender};
use std::future::Future;

use task::GenericTask;
pub use task::{Join, Task};

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

    pub fn spawn<F, O>(&self, future: F) -> impl Join<Output = O>
    where
        O: 'static + Send,
        F: Future<Output = O> + Send + 'static,
    {
        GenericTask::spawn(future, &self.sender)
    }

    pub fn run(&self) -> ! {
        let runner = Runner::new(&self.receiver);
        runner.run()
    }

    pub fn run_once(&self) {
        // TODO: implementation
        todo!()
    }
}
