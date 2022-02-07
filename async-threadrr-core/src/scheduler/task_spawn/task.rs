mod join;

use crate::utils::mpmc::Sender;
pub use join::Join;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Wake;

pub trait Task {
    fn poll<S>(self: &Arc<Self>, sender: &S)
    where
        S: Sender<T = Arc<dyn Task>>,
        Self: Sized;

    fn info(&self) -> &TaskInfo;
}

struct GenericTask<O, F, S>
where
    F: Future<Output = O> + Send + 'static,
    S: Sender<T = Arc<dyn Task>>,
{
    future: Mutex<Pin<Box<F>>>,
    sender: S,
    info: TaskInfo,
}

impl<O, F, S> GenericTask<O, F, S>
where
    F: Future<Output = O> + Send + 'static,
    S: Sender<T = Arc<dyn Task>>,
{
    fn spawn<NO, NF, NS>(future: NF, sender: &NS, info: TaskInfo) -> Arc<impl Join<Output = NO>>
    where
        NO: 'static,
        NF: Future<Output = NO> + Send + 'static,
        NS: Sender<T = Arc<dyn Task>> + 'static,
    {
        let task = Arc::new(GenericTask {
            future: Mutex::new(Box::pin(future)),
            sender: sender.clone(),
            info,
        });

        //TODO: Proper error handling
        let _ = sender.send(task);

        //TODO: Return JoinHandle
        todo!()
    }
}

impl<O, F, S> Task for GenericTask<O, F, S>
where
    F: Future<Output = O> + Send + 'static,
    S: Sender<T = Arc<dyn Task>>,
{
    fn poll<SP>(self: &Arc<Self>, _: &SP) {
        todo!()
    }

    fn info(&self) -> &TaskInfo {
        todo!()
    }
}

impl<O, F, S> Wake for GenericTask<O, F, S>
where
    F: Future<Output = O> + Send + 'static,
    S: Sender<T = Arc<dyn Task>>,
{
    fn wake(self: Arc<Self>) {
        todo!()
    }
}

pub struct TaskInfo {
    pub priority: Priority,
    pub blocking_behaviour: BlockingBehaviour,
}

enum Priority {
    INFINITE,
    HIGH,
    MEDIUM,
    LOW,
}

enum BlockingBehaviour {
    LITTLE,
    SOME,
    MUCH,
}
