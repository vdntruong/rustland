/*

mpsc = multiple producer, single consumer

tx is transmitter
rx is receiver

https://doc.rust-lang.org/std/sync/mpsc/index.html
 */
use std::sync::mpsc;
use std::thread;
use std::time;

pub fn test() {
    let (tx, rx) = mpsc::channel::<u8>();

    // tx.send(101).is_ok()
    match tx.send(101) {
        Ok(_) => println!("Send status: OK"),
        Err(_) => println!("tx impossible"),
    }
    _ = tx.send(215);

    // rx.recv() is blocking
    match rx.recv() {
        Ok(data) => println!("1- Received: {}", data),
        Err(_) => println!("rx impossible"),
    }

    match rx.recv_timeout(time::Duration::from_secs(3)) {
        Ok(data) => println!("2- Received: {}", data),
        Err(_) => println!("rx timeout"),
    }
}

pub fn test_loop() {
    let (tx, rx) = mpsc::channel::<u8>();

    for i in 1..=5 {
        let send_result = tx.send(i);
        println!("Send status: {}", send_result.is_ok());
        thread::sleep(time::Duration::from_millis(200));
    }

    let processor = move || {
        let mut failed_count = 0u8;

        loop {
            if failed_count == 3 {
                // if we fail to receive 3 times, we break, and the thread will exit
                break;
            }

            println!("Attempting to receive from channel...");
            match rx.recv_timeout(time::Duration::from_secs(3)) {
                Ok(data) => println!("Received data: {}", data),
                Err(_) => failed_count += 1,
            };
        }
    };

    _ = thread::spawn(processor).join();
    // since processor breaks when failed_count == 3, the main thread will exit
}
