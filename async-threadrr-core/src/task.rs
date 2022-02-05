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

trait WaskWake {}

struct TaskWaker {}

trait Join {}

struct JoinHandle {}
