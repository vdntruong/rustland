/*

https://doc.rust-lang.org/std/sync/struct.Mutex.html

 */
use std::ops::{Add, AddAssign};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn test() {
    let score: Mutex<u16> = Mutex::new(0u16);

    let myfunc = || {
        println!("Thread 1 is waiting for mutex lock ...");
        let mut data = score.lock().unwrap();

        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
            if i == 6 {
                drop(data); // IMPORTANT: we have to drop the lock before panicking
                            // if not, the lock will be held by the thread and the other thread will never be able to acquire it
                panic!("Error in thread 1");
            }
        }
    };
    let myfunc2 = || {
        println!("Thread 2 is waiting for mutex lock ...");
        let mut data = score.lock().unwrap();

        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 2 is adding {i}");
        }
    };

    let _ = thread::scope(|s| {
        let handle1 = s.spawn(myfunc).join();
        if handle1.is_err() {
            print!("Error happened in thread 1");
        }

        let handle2 = s.spawn(myfunc2).join();
        if handle2.is_err() {
            print!("Error happened in thread 2");
        }
    });

    println!("My age is {:?}", score.lock().unwrap());
}

pub fn test_async() {
    let score: Mutex<u16> = Mutex::new(0u16);

    let myfunc = || {
        thread::sleep(Duration::from_millis(20));
        loop {
            println!("Thread 1 is trying to mutex lock ...");
            let guard = score.try_lock();

            if guard.is_ok() {
                let mut data = guard.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("Thread 1 is adding {i}");
                }
                break;
            }

            thread::sleep(Duration::from_millis(300));
        }
    };

    let myfunc2 = || {
        println!("Thread 2 is waiting for mutex lock ...");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 2 is adding {i}");
            thread::sleep(Duration::from_millis(200));
        }
    };

    let _ = thread::scope(|s| {
        s.spawn(myfunc);
        s.spawn(myfunc2);
    });

    println!("My age is {:?}", score.lock().unwrap());
}
