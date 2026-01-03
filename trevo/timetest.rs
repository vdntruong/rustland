/*

The built-in time module in the Rust standard crate is not going to satisfy a lot of your needs
for doing things like date and time operations. This is a very restrictive as far as what you can do in here.
https://doc.rust-lang.org/std/time/index.html

So we have a 3-party called 'chrono' may be used for this purpose.
https://docs.rs/chrono/latest/chrono/

 */
use std::{time::{Duration, Instant}, ops::Sub};
use chrono::{DateTime, Local, NaiveDate, ParseResult, Utc, Weekday};

// import external crate
extern crate chrono;

pub fn test() {
    let dur: Duration = Duration::from_secs(5);
    println!("dur {} (secs)", dur.as_secs());
    println!("dur {} (millis)", dur.as_millis());
    println!("dur {} (micros)", dur.as_micros());
    println!("dur {} (nanos)", dur.as_nanos());

    let dur2: Duration = Duration::from_millis(2_000);
    println!("dur2 {} (secs)", dur2.as_secs());

    // std::ops::Sub for .sub
    let sub: Duration = dur.sub(dur2);
    println!("sub {} (secs)", sub.as_secs());

    print_title("Check sub");

    // Duration can not have negative value
    // so (2s).sub(5s) will panic

    // checked_sub will return None if the result is negative
    let sub2: Option<Duration> = dur2.checked_sub(dur);
    println!(
        "sub2 (2s - 5s): {} (secs)",
        sub2.unwrap_or_default().as_secs()
    );

    print_title("Instance time");

    let now: Instant = Instant::now();
    std::thread::sleep(Duration::from_secs(5));

    println!("time delta from now instance: {}s", now.elapsed().as_secs());
}

fn print_title(title: &str) {
    println!(" ");
    println!("--------------");
    println!("{}", title);
    println!(" ");
}

pub fn test_chrono() {
    print_title("utc time & format");
    let utc_now: DateTime<Utc> = Utc::now();
    println!("utc_now: {}", utc_now);
    // format sheet https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    println!("utc_now formatted: {}", utc_now.format("%Y-%m-%d %H:%M:%S"));

    print_title("local time");
    let local_now: DateTime<Local> = Local::now();
    println!("local_now: {}", local_now);

    print_title("naive date");
    let date1: NaiveDate = NaiveDate::from_isoywd_opt(2026, 1, Weekday::Sun).unwrap();
    println!("date1: {}", date1);

    print_title("iter days");
    date1.iter_days().take(4).for_each(|d| println!("next day: {}", d));

    print_title("date from ordinal (day number in year");
    let date2: NaiveDate = NaiveDate::from_yo_opt(2026, 31).unwrap();
    println!("date2: {}", date2.format("%Y-%m-%d (%A)"));

    print_title("date from string with template");
    let date_str = "2026|||09||15";
    let date3: ParseResult<NaiveDate> = NaiveDate::parse_from_str(date_str, "%Y|||%m||%d");
    match date3 {
        Ok(date) => println!("date3: {} from str ({})", date, date_str),
        Err(e) => eprintln!("date3: {}", e),
    }
}