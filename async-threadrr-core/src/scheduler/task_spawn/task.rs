mod join;

use crate::utils::mpmc::Sender;
pub use join::Join;
use join::{JoinHandle, Payload};
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Wake, Waker};

pub trait Task {
    fn poll<S>(self: &Arc<Self>, sender: &S)
    where
        S: Sender<T = Box<dyn Task>>,
        Self: Sized;

    fn info(&self) -> &TaskInfo;
}

trait WakeableTask: Task
where
    Self: Sized + Send,
{
    fn wake(self: Box<Self>, waker: Arc<TaskWaker<Self>>);
}

struct GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Box<dyn Task>> + 'static + Send,
{
    future: Pin<Box<F>>,
    sender: S,
    info: TaskInfo,
    payload: Arc<Mutex<Payload<O>>>,
    waker: RefCell<Option<Arc<TaskWaker<Self>>>>,
}

impl<O, F, S> GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Box<dyn Task>> + 'static + Send,
{
    fn spawn(future: F, sender: &S, info: TaskInfo) -> impl Join<Output = O> {
        let join = JoinHandle::new();
		let waker = Arc::new()

        let task = Box::new(GenericTask {
            future: Box::pin(future),
            sender: sender.clone(),
            info,
            payload: join.payload().clone(),
			waker = Cell::new()
        });

        task.schedule();

        join
    }

    fn schedule(self: Box<Self>) {
        //TODO: Proper error handling
        self.sender.clone().send(self).unwrap();
    }
}

impl<O, F, S> Task for GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Box<dyn Task>> + 'static + Send,
{
    fn poll<SP>(self: &Arc<Self>, _: &SP) {
        todo!()
    }

    fn info(&self) -> &TaskInfo {
        &self.info
    }
}

impl<O, F, S> WakeableTask for GenericTask<O, F, S>
where
    O: 'static + Send,
    F: Future<Output = O> + 'static + Send,
    S: Sender<T = Box<dyn Task>> + 'static + Send,
{
    fn wake(self: Box<Self>, waker: Arc<TaskWaker<Self>>) {
        self.waker.replace(Some(waker));
    }
}

struct TaskWaker<T>
where
    T: WakeableTask + Send,
	Self: Sync + Send
{
    task: Mutex<Option<Box<T>>>,
}

impl<T> TaskWaker<T>
where
T: WakeableTask + Send, {
	fn new() -> TaskWaker<T> {
		TaskWaker {
			task: Mutex::new(None)
		}
	}

	fn poll_and_wake(task: Box<T>) {

	}
}

impl<T> Wake for TaskWaker<T>
where
    T: WakeableTask + Send,
{
    fn wake(self: Arc<Self>) {
        self.task.unwrap().wake(self)
    }
}

pub struct TaskInfo {
    pub priority: Priority,
    pub blocking_behaviour: BlockingBehaviour,
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
