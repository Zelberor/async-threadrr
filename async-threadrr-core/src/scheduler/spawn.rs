trait TaskSpawn {
    fn schedule<F, O>(&mut self, future: F) -> JoinHandle<O>
    where
        F: Future<Output = O> + Send + 'static;

    fn recieve_task() -> Arc<Task>;
}
