/*

The main function is executed on the main thread.

https://doc.rust-lang.org/std/thread/
https://doc.rust-lang.org/std/thread/#spawning-a-thread
https://doc.rust-lang.org/book/ch16-00-concurrency.html
 */

use std::thread::{spawn, JoinHandle};

pub fn test() {
    let mut x: u128 = 0u128;
    for i in 1..5_000 {
        x += i;
    }
    println!("\x1b[32mMain thread finished, let's go check on the worker threads\x1b[0m");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x: u128 = 0u128;
        for i in 1..50_000 {
            x += i;
        }
    };

    println!("Starting new worker thread");
    let handle: JoinHandle<()> = spawn(thread_fn);
    let handle2: JoinHandle<()> = spawn(thread_fn);
    println!("The worker threads are started!");

    // join here to notice for main thread to wait for the threads to finish
    // handle.join().unwrap();
    // handle2.join().unwrap();

    loop {
        test(); // main thread
        if handle.is_finished() && handle2.is_finished() {
            println!("All threads are finished");
            break;
        }
    }
}
