
trait TaskReceive {
	fn receive_task(): Task
}

struct TaskPool {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

impl TaskPool {
    fn new() -> Self {
        let (sender, scheduled) = mpsc::channel();
        Self { sender, scheduled }
    }
}
