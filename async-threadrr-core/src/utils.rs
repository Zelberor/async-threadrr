pub mod mpmc {
    use flume;
    use std::fmt::Debug;
    pub trait Sender: Clone {
        type T;
        fn send(&self, msg: Self::T) -> Result<(), SendError<Self::T>>;
    }

    pub struct SendError<T>(pub T);

    impl<T> Debug for SendError<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("SendError").finish()
        }
    }

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
