mod task;

use std::future::Future;
use std::sync::Arc;

pub use task::Join;
pub use task::Task;

use crate::utils::mpmc::Sender;

trait TaskSpawn {
    fn spawn<F, O>(&mut self, future: F) -> Arc<dyn Join<Output = O>>
    where
        F: Future<Output = O> + Send + 'static;

    fn spawn_st<F, O>(&mut self, future: F) -> Arc<dyn Join<Output = O>>
    where
        F: Future<Output = O> + 'static;
}

struct TaskSpawner<S>
where
    S: Sender<T = Arc<dyn Task>>,
{
    sender: S,
}

impl<S> TaskSpawner<S>
where
    S: Sender<T = Arc<dyn Task>>,
{
    fn new<NS>(sender: &NS) -> TaskSpawner<NS>
    where
        NS: Sender<T = Arc<dyn Task>>,
    {
        TaskSpawner {
            sender: sender.clone(),
        }
    }
}

impl<S> TaskSpawn for TaskSpawner<S>
where
    S: Sender<T = Arc<dyn Task>>,
{
    fn spawn<F, O>(&mut self, future: F) -> Arc<dyn Join<Output = O>>
    where
        F: Future<Output = O> + Send + 'static,
    {
        todo!()
    }

    fn spawn_st<F, O>(&mut self, future: F) -> Arc<dyn Join<Output = O>>
    where
        F: Future<Output = O> + 'static,
    {
        todo!()
    }
}
