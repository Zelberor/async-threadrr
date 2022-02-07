pub mod mpmc {
    use flume;
    pub trait Sender: Clone {
        type T;
        fn send(&self, msg: Self::T) -> Result<(), SendError<Self::T>>;
    }

    pub struct SendError<T>(pub T);

    impl<T> Sender for flume::Sender<T> {
        type T = T;

        fn send(&self, msg: Self::T) -> Result<(), SendError<Self::T>> {
            match self.send(msg) {
                Ok(_) => Ok(()),
                Err(err) => Err(SendError(err.0)),
            }
        }
    }
}
