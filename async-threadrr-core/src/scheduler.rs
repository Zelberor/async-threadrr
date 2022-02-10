mod runner;
mod task_pool;
mod task_spawn;

use std::sync::Arc;

use flume::Sender;

pub use runner::Run;
use runner::Runner;
use task_pool::TaskPool;
use task_spawn::TaskSpawner;
pub use task_spawn::{Join, Task, TaskInfo, TaskSpawn};

/*pub trait Schedule: Send + Sync {
    fn spawner<'s>(&'s self) -> &'s Box<dyn TaskSpawn>;
    fn runner(&self) -> Box<dyn Run>;
}*/

pub struct SimpleScheduler {
    pool: TaskPool,
    spawner: TaskSpawner<Sender<Arc<dyn Task>>>,
}

impl SimpleScheduler {
    pub fn new() -> SimpleScheduler {
        let pool = TaskPool::new();
        let spawner = TaskSpawner::new(pool.sender());
        SimpleScheduler { pool, spawner }
    }

    pub fn spawner<'s>(&'s self) -> &'s impl TaskSpawn {
        &self.spawner
    }

    pub fn runner(&self) -> impl Run {
        Runner::new(self.pool.receiver())
    }
}
