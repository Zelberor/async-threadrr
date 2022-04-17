mod join;

use crate::utils::mpmc::Sender;
pub use join::Join;
use join::{JoinHandle, Payload};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

pub trait Task: Send + Sync {
    fn poll(self: Arc<Self>);

    fn info(&self) -> &TaskInfo;
}

pub struct GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Arc<dyn Task>> + 'static,
    Self: Sync,
{
    future: Mutex<Option<Pin<Box<F>>>>,
    sender: S,
    info: TaskInfo,
    payload: Arc<Mutex<Payload<O>>>,
}

impl<O, F, S> GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Arc<dyn Task>> + 'static,
    Self: Sync,
{
    pub fn spawn(future: F, sender: &S, info: TaskInfo) -> impl Join<Output = O> {
        let join = JoinHandle::new();

        let task = Arc::new(GenericTask {
            future: Mutex::new(Some(Box::pin(future))),
            sender: sender.clone(),
            info,
            payload: join.payload().clone(),
        });

        task.schedule();

        join
    }

    fn schedule(self: Arc<Self>) {
        //TODO: Proper error handling
        self.sender.send(self.clone()).unwrap();
    }
}

impl<O, F, S> Task for GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Arc<dyn Task>> + 'static,
    Self: Sync,
{
    fn poll(self: Arc<Self>) {
        let waker = Waker::from(self.clone());
        let mut context = Context::from_waker(&waker);
        let mut future_mutex = self.future.lock().unwrap();
        let future = future_mutex.as_mut().unwrap();
        if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
            let mut payload = self.payload.lock().unwrap();
            payload.finish(result);
        }
    }

    fn info(&self) -> &TaskInfo {
        &self.info
    }
}

impl<O, F, S> Wake for GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Arc<dyn Task>> + 'static,
    Self: Sync,
{
    fn wake(self: Arc<Self>) {
        self.schedule();
    }
}

pub struct TaskInfo {
    priority: Priority,
    blocking_behaviour: BlockingBehaviour,
}

impl TaskInfo {
    pub fn default() -> TaskInfo {
        TaskInfo {
            priority: Priority::MEDIUM,
            blocking_behaviour: BlockingBehaviour::SOME,
        }
    }
}

pub enum Priority {
    INFINITE,
    HIGH,
    MEDIUM,
    LOW,
}

pub enum BlockingBehaviour {
    LITTLE,
    SOME,
    MUCH,
}
