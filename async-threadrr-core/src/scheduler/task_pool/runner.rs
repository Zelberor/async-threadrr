use std::sync::Arc;

use super::Task;
use flume::{Receiver, RecvError, Sender, TryRecvError};

pub struct Runner {
    // TODO: Receive from other runners
    pool_receiver: Receiver<Arc<dyn Task>>,
    receiver: Receiver<Arc<dyn Task>>,
    sender: Sender<Arc<dyn Task>>,
}

impl Runner {
    pub fn new(pool_receiver: &Receiver<Arc<dyn Task>>) -> Runner {
        let (sender, receiver) = flume::unbounded();
        Runner {
            pool_receiver: pool_receiver.clone(),
            receiver,
            sender,
        }
    }
}

impl Runner {
    fn run_received(&self, r: Result<Arc<dyn Task>, RecvError>) {
        match r {
            Ok(task) => task.poll(&self.sender),
            Err(err) => panic!("Runner failed receiving task: {}", err),
        }
    }

    fn try_receive_and_run(&self, r: &Receiver<Arc<dyn Task>>) -> Result<(), ()> {
        match r.try_recv() {
            Ok(task) => {
                task.poll(&self.sender);
                Ok(())
            }
            Err(err) => match err {
                TryRecvError::Empty => Err(()),
                TryRecvError::Disconnected => panic!("Runner failed receiving task: {}", err),
            },
        }
    }

    pub fn run(&self) -> ! {
        loop {
            // Preferably receive from the own receiver
            if let Ok(_) = self.try_receive_and_run(&self.receiver) {
                continue;
            }
            // Then receive from the pool
            if let Ok(_) = self.try_receive_and_run(&self.pool_receiver) {
                continue;
            }

            // Now receive from everybody including other runners
            // TODO: Get tasks from other runners
            flume::select::Selector::new()
                .recv(&self.receiver, |r| self.run_received(r))
                .recv(&self.pool_receiver, |r| self.run_received(r))
                .wait();
        }
    }
}
