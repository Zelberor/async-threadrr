// debug
// reference with flume: 18961ms
// reference with flume(with runner pools): 18709ms
// reference with flume(with runner pools + cache optim): 17523ms

// release
// reference with flume(with runner pools): 10825ms
// reference with flume(with runner pools + cache optim): 7830ms

use async_threadrr::{Blocking, Join};
use std::{thread, time};

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

fn main() {
    const RUNNERS: usize = 16;
    async_threadrr::init(Blocking::NONE, RUNNERS);
    for _ in 0..RUNNERS {
        thread::spawn(|| {
            async_threadrr::pool(Blocking::NONE).run();
        });
    }

    let shed = async_threadrr::pool(Blocking::NONE);
    let mut joins = Vec::new();
    joins.reserve(TASKS);
    let mut numbers = Vec::new();
    numbers.reserve(TASKS);

    let start = time::SystemTime::now();
    for i in 0..TASKS {
        joins.push(shed.spawn(lel(i)));
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
