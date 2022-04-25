mod runner;
mod task;

use std::sync::{Arc, Mutex};

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
    unused_runners: Mutex<Vec<Runner>>,
}

impl TaskPool {
    pub fn new(max_runners: usize) -> Self {
        let (sender, receiver) = flume::unbounded();
        let mut pool = Self {
            sender,
            receiver,
            unused_runners: Mutex::new(Vec::new()),
        };
        pool.create_runners(max_runners);
        pool
    }

    fn create_runners(&mut self, max_runners: usize) {
        // TODO: proper error handling
        let mut unused_runners_mutex = self.unused_runners.lock().unwrap();
        let mut senders = Vec::new();
        let mut receivers = Vec::new();
        senders.reserve_exact(max_runners);
        receivers.reserve_exact(max_runners);
        unused_runners_mutex.reserve_exact(max_runners);
        for _ in 0..max_runners {
            let (sender, receiver) = flume::unbounded();
            senders.push(sender);
            receivers.push(receiver);
        }
        for i in 0..max_runners {
            let mut other_receivers = Vec::new();
            other_receivers.reserve_exact(max_runners - 1);
            for o in 0..i {
                other_receivers.push(receivers[o].clone());
            }
            for o in i + 1..max_runners {
                other_receivers.push(receivers[o].clone());
            }
            let runner = Runner::new(
                senders[i].clone(),
                receivers[i].clone(),
                self.receiver.clone(),
                other_receivers,
            );
            unused_runners_mutex.push(runner);
        }
    }

    pub fn spawn<F, O>(&self, future: F) -> impl Join<Output = O>
    where
        O: 'static + Send,
        F: Future<Output = O> + Send + 'static,
    {
        GenericTask::spawn(future, &self.sender)
    }

    pub fn run(&self) -> ! {
        // TODO: proper error handling
        let runner;
        {
            runner = self.unused_runners.lock().unwrap().pop();
        }
        match runner {
            Some(runner) => runner.run(),
            None => panic!("No runner slot available. Set max_runners to a higher value when creating the TaskPool"),
        }
    }

    pub fn run_once(&self) {
        // TODO: implementation
        todo!()
    }
}
