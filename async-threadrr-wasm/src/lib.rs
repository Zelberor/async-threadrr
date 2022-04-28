#[cfg(not(target_feature = "atomics"))]
compile_error!("The compiler features `atomics` and `bulk-memory` need to be enabled for async-threadrr-wasm to work. (Rust nightly required)");

use async_threadrr::Blocking;
use std::sync::atomic::{AtomicBool, Ordering};
use strum::EnumCount;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen(js_name = _run)]
pub fn run(blocking: Blocking) {
    async_threadrr::pool(blocking).run()
}

const _INIT_INIT: AtomicBool = AtomicBool::new(false);
static INIT: [AtomicBool; Blocking::COUNT] = [_INIT_INIT; Blocking::COUNT];

#[wasm_bindgen(module = "/src/initWorkers.js")]
extern "C" {
    #[wasm_bindgen(js_name = initWorkers, catch)]
    async fn init_workers(
        module: JsValue,
        memory: JsValue,
        blocking: Blocking,
        amount: usize,
    ) -> Result<(), JsValue>;
}

#[wasm_bindgen(module = "/src/wasmWorker.js")]
extern "C" {
    #[wasm_bindgen(js_name = _dummyFunction)]
    fn _dummy_function();
}

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
	#[wasm_bindgen(js_name = numThreads)]
	pub fn num_threads() -> usize;
}

pub async fn init_runners(blocking: Blocking, amount: usize) {
    if INIT[blocking as usize].fetch_or(true, Ordering::SeqCst) {
        return; // Runners can only be initialized once // TODO: Return error here
    };
    if amount <= 0 {
        return; // TODO: Return error here
    }
    async_threadrr::init(blocking, amount);
    let module = wasm_bindgen::module();
    let memory = wasm_bindgen::memory();
    init_workers(module, memory, blocking, amount).await;
}
