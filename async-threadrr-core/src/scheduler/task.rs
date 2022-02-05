use std::future::Future;
use std::pin::Pin;
use std::sync::{mpsc, Arc, Mutex};

trait Task {}

struct Task<O> {
    future: Mutex<Pin<Box<dyn Future<Output = O> + Send>>>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl<O> Task<O> {
    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

struct TaskInfo {}

trait Join {}

struct JoinHandle<O> {}
