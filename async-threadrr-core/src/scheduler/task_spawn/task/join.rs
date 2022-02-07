use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex, TryLockError};
use std::task::{Context, Poll, Waker};

pub trait Join: Future {
    fn join(self) -> Self::Output;
}

pub struct JoinHandle<O> {
    condvar: Arc<Condvar>,
    payload: Arc<Mutex<Payload<O>>>,
}

impl<O> JoinHandle<O> {
    fn new() -> JoinHandle<O> {
        let condvar = Arc::new(Condvar::new());
        let payload = Payload::new(condvar.clone());
        JoinHandle {
            condvar,
            payload: Arc::new(Mutex::new(payload)),
        }
    }

    fn payload(&self) -> Arc<Payload<O>> {
        self.payload().clone()
    }
}

impl<O> Join for JoinHandle<O> {
    fn join(self) -> Self::Output {
        let mut payload = self.payload.lock().unwrap();
        while payload.result.is_none() {
            let payload = self.condvar.wait(payload).unwrap();
        }
        payload.result.unwrap()
        // TODO: Error handling / panic messages
    }
}

impl<O> Future for JoinHandle<O> {
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.payload.try_lock() {
            Ok(payload) => match payload.result {
                Some(result) => Poll::Ready(result),
                None => {
                    payload.waker = Some(cx.waker().clone());
                    Poll::Pending
                } // No result yet: save the waker in the payload so the task can wake this future when finished
            }, // payload currently not used
            Err(err) => match err {
                TryLockError::Poisoned(err) => panic!("{}", err),
                TryLockError::WouldBlock => {
                    cx.waker().wake();
                    Poll::Pending
                } // Apparently the task is currently writing the result, so wake this future and try again (busy loop)
            },
        }
    }
}

struct Payload<O> {
    result: Option<O>,
    waker: Option<Waker>,
    notifier: Arc<Condvar>,
}

impl<O> Payload<O> {
    fn finished(self: &Arc<Self>, result: O) {
        self.result = Some(result);
        if let Some(waker) = self.waker {
            waker.wake();
        }
        self.notifier.notify_all();
    }

    fn new(notifier: Arc<Condvar>) -> Payload<O> {
        Payload {
            result: None,
            waker: None,
            notifier,
        }
    }
}
