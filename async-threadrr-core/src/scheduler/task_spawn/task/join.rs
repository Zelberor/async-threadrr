use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex, TryLockError};
use std::task::{Context, Poll, Waker};

pub trait Join: Future {
    fn join(&self) -> Self::Output;
}

pub struct JoinHandle<O> {
    condvar: Arc<Condvar>,
    payload: Arc<Mutex<Payload<O>>>,
}

impl<O> JoinHandle<O> {
    pub fn new() -> JoinHandle<O> {
        let condvar = Arc::new(Condvar::new());
        let payload = Payload::new(condvar.clone());
        JoinHandle {
            condvar,
            payload: Arc::new(Mutex::new(payload)),
        }
    }

    pub fn payload(&self) -> &Arc<Mutex<Payload<O>>> {
        &self.payload
    }
}

impl<O> Join for JoinHandle<O> {
    fn join(&self) -> Self::Output {
        let mut payload = self.payload.lock().unwrap();
        while payload.result.is_none() {
            payload = self.condvar.wait(payload).unwrap();
        }
        payload.result.take().unwrap()
        // TODO: Error handling / panic messages
    }
}

impl<O> Future for JoinHandle<O> {
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.payload.try_lock() {
            Ok(mut payload) => match payload.result.take() {
                Some(result) => Poll::Ready(result),
                None => {
                    payload.waker = Some(cx.waker().clone());
                    Poll::Pending
                } // No result yet: save the waker in the payload so the task can wake this future when finished
            }, // payload currently not used
            Err(err) => match err {
                TryLockError::Poisoned(err) => panic!("{}", err),
                TryLockError::WouldBlock => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                } // Apparently the task is currently writing the result, so wake this future and try again (busy loop)
            },
        }
    }
}

pub struct Payload<O> {
    result: Option<O>,
    waker: Option<Waker>,
    notifier: Arc<Condvar>,
    got_result: bool,
    // TODO: Panic when joined / polled again
}

impl<O> Payload<O> {
    pub fn finish(&mut self, result: O) {
        self.result = Some(result);
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        self.notifier.notify_all();
    }

    fn new(notifier: Arc<Condvar>) -> Payload<O> {
        Payload {
            result: None,
            waker: None,
            notifier,
            got_result: false,
        }
    }
}
