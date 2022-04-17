use async_threadrr_core::scheduler::{Run, TaskSpawn};
use std::thread;

async fn lel(num: usize) -> usize {
    //println!("NUM: {}, ID: {:?}", num, thread::current().id());
    num + 1
}

/*async fn fib(n: u64) -> u64 {
    match n {
        1 => 1,
        2 => 1,
        _ => {
            let spawner = async_threadrr_core::spawner();
            //let test = fib(n - 1);
            //let n_minus_1 = spawner.spawn(test, None);
            //let n_minus_2 = spawner.spawn(fib(n - 2), None);
            fib(n - 1).await + fib(n - 2).await
        }
    }
}*/

const TASKS: usize = 1000000;

fn main() {
    for _ in 0..16 {
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
