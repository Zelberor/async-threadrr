use async_threadrr_core::scheduler::{Run, TaskSpawn};
use core::num;
use std::thread;

async fn lel(num: usize) -> usize {
    //println!("NUM: {}, ID: {:?}", num, thread::current().id());
    num + 1
}

const TASKS: usize = 10000000;

fn main() {
    for _ in 0..8 {
        thread::spawn(|| {
            let runner = async_threadrr_core::runner();
            runner.run();
        });
    }

    let spawner = async_threadrr_core::spawner();
    let mut joins = Vec::new();
    joins.reserve(TASKS);
    let mut numbers = Vec::new();
    numbers.reserve(TASKS);
    for i in 0..=TASKS {
        joins.push(spawner.spawn(lel(i), None));
    }
    println!("Spawning done.");
    while let Some(item) = joins.pop() {
        numbers.push(item.join());
    }
    println!("Done.");
    while let Some(number) = numbers.pop() {
        print!("{} ", number);
    }
    print!("\n");
}
