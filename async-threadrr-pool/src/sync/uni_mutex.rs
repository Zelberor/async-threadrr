mod block_mutex;

use std::collections::VecDeque;
use std::future::Future;
use std::ops::Deref;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::Waker;
use std::task::{Context, Poll};

use block_mutex::{
    BlockLockResult, BlockMutex, BlockMutexGuard, BlockPoisonError, BlockTryLockError,
    BlockTryLockResult, SpinLock,
};

pub type PoisonError<Guard> = BlockPoisonError<Guard>;
pub type TryLockError<Guard> = BlockTryLockError<Guard>;
pub type LockResult<Guard> = BlockLockResult<Guard>;
pub type TryLockResult<Guard> = BlockTryLockResult<Guard>;

pub struct Mutex<T>
where
    T: ?Sized,
{
    waiting_wakers: BlockMutex<VecDeque<Waker>>, // Wakers of waiting async tasks
    mutex: BlockMutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        Mutex {
            waiting_wakers: BlockMutex::new(VecDeque::new()),
            mutex: BlockMutex::new(t),
        }
    }
    pub fn into_inner(self) -> LockResult<T> {
        self.mutex.into_inner()
    }
}

impl<T> Mutex<T>
where
    T: ?Sized,
{
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        match self.mutex.lock() {
            Ok(guard) => Ok(MutexGuard::new(self, guard)),
            Err(poison) => Err(PoisonError::new(MutexGuard::new(self, poison.into_inner()))),
        }
    }
    pub fn lock_async(&self) -> FutureLock<'_, T> {
        FutureLock::new(self)
    }

    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
        match self.mutex.try_lock() {
            Ok(guard) => Ok(MutexGuard::new(self, guard)),
            Err(err) => match err {
                TryLockError::Poisoned(poison) => Err(TryLockError::Poisoned(PoisonError::new(
                    MutexGuard::new(self, poison.into_inner()),
                ))),
                TryLockError::WouldBlock => Err(TryLockError::WouldBlock),
            },
        }
    }

    pub fn spin_lock(&self) -> LockResult<MutexGuard<'_, T>> {
        loop {
            let result = self.try_lock();
            match result {
                Ok(guard) => return Ok(guard),
                Err(err) => match err {
                    BlockTryLockError::Poisoned(poison) => return Err(poison),
                    BlockTryLockError::WouldBlock => (),
                },
            }
        }
    }

    pub fn is_poisoned(&self) -> bool {
        self.mutex.is_poisoned()
    }

    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        self.mutex.get_mut()
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(t: T) -> Self {
        Mutex::new(t)
    }
}

//TODO: Default + Debug

#[must_use = "if unused the Mutex will immediately unlock"]
/*#[must_not_suspend = "holding a MutexGuard across suspend \
points can cause deadlocks, delays, \
and cause Futures to not implement `Send`"] // TODO: Enable when feature is stabilized*/
pub struct MutexGuard<'mutex, T>
where
    T: ?Sized + 'mutex,
{
    mutex: &'mutex Mutex<T>,
    guard: BlockMutexGuard<'mutex, T>,
}

impl<'mutex, T> MutexGuard<'mutex, T>
where
    T: ?Sized,
{
    fn new(mutex: &'mutex Mutex<T>, guard: BlockMutexGuard<'mutex, T>) -> MutexGuard<'mutex, T> {
        MutexGuard { mutex, guard }
    }
}

impl<T> Deref for MutexGuard<'_, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<T> DerefMut for MutexGuard<'_, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<T> Drop for MutexGuard<'_, T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        let mut wakers_mutex = self.mutex.waiting_wakers.spin_lock().unwrap(); // No error handling, since this lock should never be poisoned
        if let Some(waker) = wakers_mutex.pop_front() {
            waker.wake();
        }
    }
}

pub struct FutureLock<'mutex, T>
where
    T: ?Sized + 'mutex,
{
    mutex: &'mutex Mutex<T>,
}

impl<'mutex, T> FutureLock<'mutex, T>
where
    T: ?Sized,
{
    fn new(mutex: &'mutex Mutex<T>) -> FutureLock<'mutex, T> {
        FutureLock { mutex }
    }
}

impl<'mutex, T> Future for FutureLock<'mutex, T> {
    type Output = LockResult<MutexGuard<'mutex, T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.mutex.try_lock() {
            Ok(guard) => Poll::Ready(Ok(guard)),
            Err(err) => match err {
                TryLockError::Poisoned(poison) => Poll::Ready(Err(poison)),
                TryLockError::WouldBlock => {
                    let mut wakers_mutex = self.mutex.waiting_wakers.spin_lock().unwrap(); // No error handling, since this lock should never be poisoned
                    wakers_mutex.push_back(cx.waker().clone());
                    Poll::Pending
                }
            },
        }
    }
}

//TODO: Debug + Display
