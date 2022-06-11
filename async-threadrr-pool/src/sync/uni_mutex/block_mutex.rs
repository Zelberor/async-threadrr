use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use std::sync::TryLockError;
use std::sync::TryLockResult;

pub type BlockLockResult<Guard> = LockResult<Guard>;
pub type BlockMutex<T> = Mutex<T>;
pub type BlockMutexGuard<'a, T> = MutexGuard<'a, T>;
pub type BlockPoisonError<Guard> = PoisonError<Guard>;
pub type BlockTryLockError<Guard> = TryLockError<Guard>;
pub type BlockTryLockResult<Guard> = TryLockResult<Guard>;

pub trait SpinLock {
    type T;

    fn spin_lock<'a>(&'a self) -> BlockLockResult<BlockMutexGuard<'a, Self::T>>;
}

impl<T> SpinLock for BlockMutex<T> {
    type T = T;

    fn spin_lock<'a>(&'a self) -> BlockLockResult<BlockMutexGuard<'a, Self::T>> {
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
}
