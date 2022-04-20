// debug
// reference with flume: 18961ms
// reference with flume(with runner pools): 18709ms
// reference with flume(with runner pools + cache optim): 17523ms

// release
// reference with flume(with runner pools): 10825ms
// reference with flume(with runner pools + cache optim): 7830ms

use async_threadrr_core::scheduler::Join;
use std::{thread, time};

async fn inc(num: usize) -> usize {
    num + 1
}

async fn lel(num: usize) -> usize {
    let mut number = inc(num).await;

    let shed = async_threadrr_core::scheduler();
    let mut joins = Vec::new();
    joins.reserve(INT_TASKS);
    for i in 0..INT_TASKS {
        joins.push(shed.spawn(inc(i), None));
    }
    while let Some(item) = joins.pop() {
        number += item.await;
    }
    number
}

const TASKS: usize = 1000;
const INT_TASKS: usize = 10000;

fn main() {
    for _ in 0..16 {
        thread::spawn(|| {
            async_threadrr_core::scheduler().run();
        });
    }

    let shed = async_threadrr_core::scheduler();
    let mut joins = Vec::new();
    joins.reserve(TASKS);
    let mut numbers = Vec::new();
    numbers.reserve(TASKS);

    let start = time::SystemTime::now();
    for i in 0..TASKS {
        joins.push(shed.spawn(lel(i), None));
    }
    while let Some(item) = joins.pop() {
        numbers.push(item.join());
    }
    let duration = time::SystemTime::now().duration_since(start).unwrap();
    println!("Done.");
    while let Some(number) = numbers.pop() {
        print!("{} ", number);
    }
    print!("\n");
    println!("Duration: {}", duration.as_millis());
}
