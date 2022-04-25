use std::sync::Arc;

use super::Task;
use flume::{Receiver, RecvError, Sender, TryRecvError};

pub struct Runner {
    pool_receiver: Receiver<Arc<dyn Task>>,
    other_receivers: Vec<Receiver<Arc<dyn Task>>>,
    receiver: Receiver<Arc<dyn Task>>,
    sender: Sender<Arc<dyn Task>>,
}

impl Runner {
    pub fn new(
        self_sender: Sender<Arc<dyn Task>>,
        self_receiver: Receiver<Arc<dyn Task>>,
        pool_receiver: Receiver<Arc<dyn Task>>,
        other_receivers: Vec<Receiver<Arc<dyn Task>>>,
    ) -> Runner {
        Runner {
            pool_receiver,
            receiver: self_receiver,
            sender: self_sender,
            other_receivers,
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
            // Preferably try to receive from the own receiver
            if let Ok(_) = self.try_receive_and_run(&self.receiver) {
                continue;
            }
            // Then try to receive from the pool
            if let Ok(_) = self.try_receive_and_run(&self.pool_receiver) {
                continue;
            }

            // Now receive from everybody including other runners
            let mut selector = flume::select::Selector::new()
                .recv(&self.receiver, |r| self.run_received(r))
                .recv(&self.pool_receiver, |r| self.run_received(r));
            for receiver in self.other_receivers.iter() {
                selector = selector.recv(receiver, |r| self.run_received(r));
            }
            selector.wait();
        }
    }
}
