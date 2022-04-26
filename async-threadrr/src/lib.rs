pub use async_threadrr_pool::Join;
use async_threadrr_pool::TaskPool;
use std::sync::Once;

pub enum Blocking {
    NONE,
    SOME,
    MUCH,
}

static mut TASKPOOL_NO_BLOCKING: Option<TaskPool> = None;
static INIT_NO_BLOCKING: Once = Once::new();
static mut TASKPOOL_SOME_BLOCKING: Option<TaskPool> = None;
static INIT_SOME_BLOCKING: Once = Once::new();
static mut TASKPOOL_MUCH_BLOCKING: Option<TaskPool> = None;
static INIT_MUCH_BLOCKING: Once = Once::new();

fn get_initialized() -> &'static TaskPool {
    if INIT_MUCH_BLOCKING.is_completed() {
        unsafe { TASKPOOL_MUCH_BLOCKING.as_ref().unwrap() }
    } else if INIT_SOME_BLOCKING.is_completed() {
        unsafe { TASKPOOL_SOME_BLOCKING.as_ref().unwrap() }
    } else if INIT_NO_BLOCKING.is_completed() {
        unsafe { TASKPOOL_NO_BLOCKING.as_ref().unwrap() }
    } else {
        panic!("No task pool initialized")
    }
}

pub fn pool(blocking: Blocking) -> &'static TaskPool {
    match blocking {
        Blocking::NONE => {
            if INIT_NO_BLOCKING.is_completed() {
                unsafe { TASKPOOL_NO_BLOCKING.as_ref().unwrap() }
            } else {
                get_initialized()
            }
        }
        Blocking::SOME => {
            if INIT_SOME_BLOCKING.is_completed() {
                unsafe { TASKPOOL_SOME_BLOCKING.as_ref().unwrap() }
            } else {
                get_initialized()
            }
        }
        Blocking::MUCH => {
            if INIT_MUCH_BLOCKING.is_completed() {
                unsafe { TASKPOOL_MUCH_BLOCKING.as_ref().unwrap() }
            } else {
                get_initialized()
            }
        }
    }
}

pub fn init(blocking: Blocking, max_runners: usize) {
    match blocking {
        Blocking::NONE => INIT_NO_BLOCKING.call_once(|| {
            unsafe {
                TASKPOOL_NO_BLOCKING = Some(TaskPool::new(max_runners));
            };
        }),
        Blocking::SOME => INIT_SOME_BLOCKING.call_once(|| {
            unsafe {
                TASKPOOL_SOME_BLOCKING = Some(TaskPool::new(max_runners));
            };
        }),
        Blocking::MUCH => INIT_MUCH_BLOCKING.call_once(|| {
            unsafe {
                TASKPOOL_MUCH_BLOCKING = Some(TaskPool::new(max_runners));
            };
        }),
    }
}
