mod utils;

use async_threadrr::{Blocking, Join};
use async_threadrr_wasm;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub async fn start() {
    utils::set_panic_hook();
    async_threadrr_wasm::init_runners(5, 5, 5).await;
}

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
    fn log(msg: &str);
}

async fn inc(num: usize) -> usize {
    num + 1
}

async fn lel(num: usize) -> usize {
    let mut number = inc(num).await;

    let shed = async_threadrr::pool(Blocking::NONE);
    let mut joins = Vec::new();
    joins.reserve(INT_TASKS);
    for i in 0..INT_TASKS {
        joins.push(shed.spawn(inc(i)));
    }
    while let Some(item) = joins.pop() {
        number += item.await;
    }
    number
}

const TASKS: usize = 1000;
const INT_TASKS: usize = 10000;

#[wasm_bindgen]
pub async fn test() {
    log("Starting test...");
    let shed = async_threadrr::pool(Blocking::NONE);
    let mut joins = Vec::new();
    joins.reserve(TASKS);
    let mut numbers = Vec::new();
    numbers.reserve(TASKS);

    //let start = time::SystemTime::now();
    for i in 0..TASKS {
        joins.push(shed.spawn(lel(i)));
    }
    while let Some(item) = joins.pop() {
        numbers.push(item.await);
    }
    //let duration = time::SystemTime::now().duration_since(start).unwrap();
    log("Done.");
    while let Some(number) = numbers.pop() {
        log(format!("{} ", number).as_str());
    }
    //println!("Duration: {}", duration.as_millis());
}
