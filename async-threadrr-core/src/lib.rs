pub mod scheduler;
mod utils;

use std::sync::Once;

use scheduler::{Run, SimpleScheduler, TaskSpawn};

static mut GLOBAL_SCHEDULER: Option<SimpleScheduler> = None;
static INIT: Once = Once::new();

pub fn init() {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_SCHEDULER = Some(SimpleScheduler::new());
        });
    }
}

pub fn is_initialized() -> bool {
    INIT.is_completed()
}

pub fn spawner() -> &'static impl TaskSpawn {
    init();
    unsafe { GLOBAL_SCHEDULER.as_ref().unwrap().spawner() }
}

pub fn runner() -> impl Run {
    init();
    unsafe { GLOBAL_SCHEDULER.as_ref().unwrap().runner() }
}
