pub use async_threadrr_pool::Join;
use async_threadrr_pool::TaskPool;
use std::sync::Once;

static mut TASKPOOL_NO_BLOCKING: Option<TaskPool> = None;
static INIT_NO_BLOCKING: Once = Once::new();
static mut TASKPOOL_SOME_BLOCKING: Option<TaskPool> = None;
static INIT_SOME_BLOCKING: Once = Once::new();
static mut TASKPOOL_MUCH_BLOCKING: Option<TaskPool> = None;
static INIT_MUCH_BLOCKING: Once = Once::new();

pub fn init_no_blocking(max_runners: usize) {
    INIT_NO_BLOCKING.call_once(|| {
        unsafe {
            TASKPOOL_NO_BLOCKING = Some(TaskPool::new(max_runners));
        };
    })
}

pub fn init_some_blocking(max_runners: usize) {
    INIT_SOME_BLOCKING.call_once(|| {
        unsafe {
            TASKPOOL_SOME_BLOCKING = Some(TaskPool::new(max_runners));
        };
    })
}

pub fn init_much_blocking(max_runners: usize) {
    INIT_MUCH_BLOCKING.call_once(|| {
        unsafe {
            TASKPOOL_MUCH_BLOCKING = Some(TaskPool::new(max_runners));
        };
    })
}

pub fn no_blocking() -> &'static TaskPool {
    if INIT_NO_BLOCKING.is_completed() {
        unsafe { TASKPOOL_NO_BLOCKING.as_ref().unwrap() }
    } else {
        get_initialized()
    }
}

pub fn some_blocking() -> &'static TaskPool {
    if INIT_SOME_BLOCKING.is_completed() {
        unsafe { TASKPOOL_SOME_BLOCKING.as_ref().unwrap() }
    } else {
        get_initialized()
    }
}

pub fn much_blocking() -> &'static TaskPool {
    if INIT_MUCH_BLOCKING.is_completed() {
        unsafe { TASKPOOL_MUCH_BLOCKING.as_ref().unwrap() }
    } else {
        get_initialized()
    }
}

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
