pub use async_threadrr_pool::Join;
use async_threadrr_pool::TaskPool;
use std::sync::Once;
use strum::EnumCount;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(EnumCount, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum Blocking {
    MUCH = 0,
    SOME = 1,
    NONE = 2,
}

const _POOLS_INIT: Option<TaskPool> = None;
static mut POOLS: [Option<TaskPool>; Blocking::COUNT] = [_POOLS_INIT; Blocking::COUNT];
const _INITS_INIT: Once = Once::new();
static INITS: [Once; Blocking::COUNT] = [_INITS_INIT; Blocking::COUNT];

fn get_initialized() -> &'static TaskPool {
    for i in 0..Blocking::COUNT {
        if INITS[i].is_completed() {
            unsafe {
                return POOLS[i].as_ref().unwrap();
            }
        }
    }
    panic!("No task pool initialized")
}

pub fn pool(blocking: Blocking) -> &'static TaskPool {
    if INITS[blocking as usize].is_completed() {
        unsafe { POOLS[blocking as usize].as_ref().unwrap() }
    } else {
        get_initialized()
    }
}

pub fn init(blocking: Blocking, max_runners: usize) {
    INITS[blocking as usize].call_once(|| {
        unsafe {
            POOLS[blocking as usize] = Some(TaskPool::new(max_runners));
        };
    });
}
