mod task_pool;

use std::future::Future;

pub use task_pool::Join;
use task_pool::TaskPool;

pub struct SimpleScheduler {
    pool: TaskPool,
}

impl SimpleScheduler {
    pub fn new() -> SimpleScheduler {
        let pool = TaskPool::new();
        SimpleScheduler { pool }
    }

    pub fn spawn<F, O>(
        &self,
        future: F,
        task_info: Option<Box<dyn TaskInfo>>,
    ) -> impl Join<Output = O>
    where
        O: 'static + Send,
        F: Future<Output = O> + Send + 'static,
    {
        /*let info = match task_info {
            Some(info) => info,
            None => I::default(),
        };*/

        self.pool.spawn(future)
    }

    pub fn run(&self) -> ! {
        self.pool.run();
    }

    pub fn run_once(&self) {
        self.pool.run_once();
    }
}

pub trait TaskInfo {
    fn blocking_behaviour(&self) -> BlockingBehaviour;
}

pub struct TaskInfoV1 {
    blocking_behaviour: BlockingBehaviour,
}

impl TaskInfo for TaskInfoV1 {
    fn blocking_behaviour(&self) -> BlockingBehaviour {
        self.blocking_behaviour
    }
}

#[derive(Clone, Copy)]
pub enum BlockingBehaviour {
    NONE,
    SOME,
    MUCH,
}
