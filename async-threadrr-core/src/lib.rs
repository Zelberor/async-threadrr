pub mod scheduler;
mod utils;

use std::sync::Once;

use scheduler::SimpleScheduler;

static mut GLOBAL_SCHEDULER: Option<SimpleScheduler> = None;
static INIT: Once = Once::new();

fn init() {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_SCHEDULER = Some(SimpleScheduler::new());
        });
    }
}

pub fn scheduler() -> &'static SimpleScheduler {
    init();
    unsafe { GLOBAL_SCHEDULER.as_ref().unwrap() }
}
