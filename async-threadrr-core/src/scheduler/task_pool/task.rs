mod join;

use flume::Sender;
pub use join::Join;
use join::{JoinHandle, Payload};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

pub trait Task: Send + Sync {
    fn poll(self: Arc<Self>, sender: &Sender<Arc<dyn Task>>);
}

pub struct GenericTask<O, F>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    Self: Sync,
{
    future: Mutex<Pin<Box<F>>>,
    payload: Arc<Mutex<Payload<O>>>,
    wake_sender: Mutex<Option<Sender<Arc<dyn Task>>>>,
}

impl<O, F> GenericTask<O, F>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    Self: Sync,
{
    pub fn spawn(future: F, sender: &Sender<Arc<dyn Task>>) -> impl Join<Output = O> {
        let join = JoinHandle::new();
        let task = Arc::new(GenericTask {
            future: Mutex::new(Box::pin(future)),
            payload: join.payload().clone(),
            wake_sender: Mutex::new(None),
        });

        task.schedule(sender);

        join
    }

    fn schedule(self: Arc<Self>, sender: &Sender<Arc<dyn Task>>) {
        // TODO: proper error handling
        sender.send(self).unwrap();
    }
}

impl<O, F> Task for GenericTask<O, F>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    Self: Sync,
{
    fn poll(self: Arc<Self>, sender: &Sender<Arc<dyn Task>>) {
        // TODO: proper error handling
        // Setup waker
        let waker = Waker::from(self.clone());
        let mut context = Context::from_waker(&waker);
        {
            let mut wake_sender_mutex = self.wake_sender.lock().unwrap();
            *wake_sender_mutex = Some(sender.clone());
        }

        let mut future_mutex = self.future.lock().unwrap();
        // Poll
        if let Poll::Ready(result) = future_mutex.as_mut().poll(&mut context) {
            let mut payload = self.payload.lock().unwrap();
            payload.finish(result);
        }
    }
}

impl<O, F> Wake for GenericTask<O, F>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    Self: Sync,
{
    fn wake(self: Arc<Self>) {
        let sender;
        {
            // TODO: proper error handling
            let mut wake_sender_mutex = self.wake_sender.lock().unwrap();
            sender = wake_sender_mutex.take().unwrap();
        }
        self.schedule(&sender);
    }
}
