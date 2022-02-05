use std::future::Future;
use std::pin::Pin;
use std::sync::{mpsc, Arc, Mutex};

trait Schedule {
    fn schedule<F, O>(&mut self, future: F) -> JoinHandle<O>
    where
        F: Future<Output = O> + Send + 'static;

    fn recieve_task() -> Arc<Task>;
}

struct SchedulerHandle {}

struct JoinHandle<O> {}

struct Scheduler {
    pool: TaskPool,
}

struct Runner {}
