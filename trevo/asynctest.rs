/*

async is about future things!

The Future trait

To work with async code we need an executor
(which is not in std - so we have to use a 3-party crate)
In this example we use smol

https://rust-lang.github.io/async-book/
https://doc.rust-lang.org/book/ch17-00-async-await.html
https://rust-lang.github.io/async-book/02_execution/01_chapter.html
 */

use futures::future::FutureExt;
// use futures::future::join;
use futures::{join, pin_mut, select};
// use std::future::Future;

pub fn test() {
    let num = get_number();

    /*
       To handle a Future result we need an Executor invoked
       the executor help to poll the future to completion
       if not, the future will never be completed
    */
    let result = smol::block_on(num);
    println!("num {}", result);

    println!();
    // execute multiple futures

    let num1 = get_number1();
    let num2 = get_number2();
    let num3 = get_number3();
    let result = smol::block_on(
        async{
            join!(num1, num2, num3)
        }
    );
    println!("{:#?}", result);

    println!();
    // execute multiple futures 2

    let num1 = get_number1().fuse();
    let num2 = get_number2().fuse();
    let num3 = get_number3().fuse();
    pin_mut!(num1, num2, num3);

    smol::block_on(async {
        loop {
            select! {
                x = num1 => println!("ok num1 {:?}", x),
                x = num2 => println!("ok num2 {:?}", x),
                x = num3 => println!("ok num3 {:?}", x),
                complete => {
                    println!("all completed!");
                    break;
                },
            };
        }
    });
}

// fn get_number() -> impl Feature<u8>
async fn get_number() -> u8 {
    println!("get_number");
    8
}

async fn get_number1() -> u8 {
    println!("get_number1");
    std::thread::sleep(std::time::Duration::from_millis(10));
    10
}

async fn get_number2() -> u8 {
    println!("get_number2");
    std::thread::sleep(std::time::Duration::from_millis(200));
    15
}

async fn get_number3() -> u8 {
    println!("get_number3");
    std::thread::sleep(std::time::Duration::from_millis(300));
    20
}
