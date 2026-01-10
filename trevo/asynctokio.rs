/*

async is concurrent just like async/await

I'll use tokio in single thread and multi thread
- Enable multi threading

https://tokio.rs/

attribute macros:
- main
- test

# Install
rt - runtime

cargo add tokio --features macros,rt,rt-multi-thread
 */
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::Duration;

/*
#[tokio::main(flavor = "current_thread")] // single thread required feature rt
#[tokio::main(flavor = "multi_thread")] // multi thread required feature rt-multi-thread
#[tokio::main(flavor = "multi_thread", worker_threads = 4)] // specify number of worker threads
 */
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
pub async fn test() {
    my_async_fn().await; // 'await' that tell tokio to wait for the future to complete

    let racer = F1Racer::new("Pete".to_string());
    let best_lap_time = racer.await;

    println!("best lap time was: {}", best_lap_time);
    let racer2 = F1Racer::new("Bill".to_string());
    let racer3 = F1Racer::new("David".to_string());
    let handle01 = tokio::task::spawn(racer2);
    let handle02 = tokio::task::spawn(racer3);
    loop {
        if handle01.is_finished() && handle02.is_finished() {
            println!("all racer finished");
            break;
        }
        sleep(Duration::from_millis(300));
    }
}

async fn my_async_fn() {
    sleep(Duration::from_secs(1));
    println!("hello from tokio");
}

struct F1Racer {
    name: String,
    completed_lab: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}

impl F1Racer {
    fn new(name: String) -> Self {
        F1Racer {
            name,
            completed_lab: 2,
            laps: 5,
            best_lap_time: 140,
            lap_times: vec![87u8, 54, 67, 89, 31],
        }
    }

    fn do_lap(&mut self) {
        println!("doing a new lap...");
        let lap_time = self.lap_times.pop();
        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }
        self.completed_lab += 1;
    }
}

// custom struct that implements Future trait

/*
the Future trait is required to be implemented for the type to be used with async/await

core components
- output: is the return type of the future
- poll: is the method that is called to execute the future
  - pending: the future is not ready yet
  - ready: the future is ready
  - cx.waker(): that is used to wake up the task when the future is ready
 */
impl Future for F1Racer {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Thread assigned is ID: {:?}", std::thread::current().id());

        if self.completed_lab < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref(); // tell tokio to wake up the task when the future is ready to check if it's ready
            return Poll::Pending;
        }

        println!("racer {} is done all laps", self.name);
        println!(
            "racer {} with best lap time: {}",
            self.name, self.best_lap_time
        );
        Poll::Ready(self.best_lap_time)
    }
}
