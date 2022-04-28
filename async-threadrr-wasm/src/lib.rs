#[cfg(not(target_feature = "atomics"))]
compile_error!("The compiler features `atomics` and `bulk-memory` need to be enabled for async-threadrr-wasm to work. (Rust nightly required)");

use async_threadrr::Blocking;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen(js_name = _runNoneBlocking)]
pub fn run_none_blocking() {
    async_threadrr::pool(Blocking::NONE).run()
}

#[wasm_bindgen(js_name = _runSomeBlocking)]
pub fn run_some_blocking() {
    async_threadrr::pool(Blocking::SOME).run()
}

#[wasm_bindgen(js_name = _runMuchBlocking)]
pub fn run_much_blocking() {
    async_threadrr::pool(Blocking::MUCH).run()
}

static INIT: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen(module = "/src/initWorkers.js")]
extern "C" {
    #[wasm_bindgen(js_name = initWorkers, catch)]
    async fn init_workers(
        module: JsValue,
        memory: JsValue,
        no_blocking_amount: usize,
        some_blocking_amount: usize,
        much_blocking_amount: usize,
    ) -> Result<(), JsValue>;
}

#[wasm_bindgen(module = "/src/wasmWorker.js")]
extern "C" {
	#[wasm_bindgen(js_name = _dummyFunction)]
	fn dummy_function();
}

pub async fn init_runners(
    no_blocking_amount: usize,
    some_blocking_amount: usize,
    much_blocking_amount: usize,
) -> Result<(), JsValue> {
    if INIT.fetch_or(true, Ordering::SeqCst) {
        return Err(JsError::new("Runners can only be initialized once.").into());
    };
    if no_blocking_amount > 0 {
        async_threadrr::init(Blocking::NONE, no_blocking_amount);
    }
    let module = wasm_bindgen::module();
    let memory = wasm_bindgen::memory();
    init_workers(
        module,
        memory,
        no_blocking_amount,
        some_blocking_amount,
        much_blocking_amount,
    )
    .await
}
