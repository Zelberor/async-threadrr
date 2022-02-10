mod task;

use std::future::Future;
use std::sync::Arc;

use task::GenericTask;
pub use task::{Join, Task, TaskInfo};

use crate::utils::mpmc::Sender;

pub trait TaskSpawn: Clone {
    fn spawn<F, O>(&self, future: F, task_info: Option<TaskInfo>) -> Box<dyn Join<Output = O>>
    where
        O: 'static + Send,
        F: Future<Output = O> + 'static + Send;
}

#[derive(Clone)]
pub struct TaskSpawner<S>
where
    S: Sender<T = Arc<dyn Task>>,
{
    sender: S,
}

impl<S> TaskSpawner<S>
where
    S: Sender<T = Arc<dyn Task>>,
{
    pub fn new(sender: &S) -> TaskSpawner<S> {
        TaskSpawner {
            sender: sender.clone(),
        }
    }
}

impl<S> TaskSpawn for TaskSpawner<S>
where
    S: Sender<T = Arc<dyn Task>> + 'static,
{
    fn spawn<F, O>(&self, future: F, task_info: Option<TaskInfo>) -> Box<dyn Join<Output = O>>
    where
        O: 'static + Send,
        F: Future<Output = O> + Send + 'static,
    {
        let info = match task_info {
            Some(info) => info,
            None => TaskInfo::default(),
        };

        Box::new(GenericTask::spawn(future, &self.sender, info))
    }
}
