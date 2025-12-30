use num_traits::{Float, ToPrimitive};

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

// "Float" is a trait provided by num_traits crate
// Here it is being used as a type constraint / trait bound for generic type T
// This means that T must implement the Float trait
fn common_solve<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn super_solve<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn util_solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a_f32: f32 = 3.0;
    let b_f64: f64 = 4.0;

    // let a_f64 = a as f64;   // cast directly
    let a_f64 = a_f32.to_f64().unwrap(); // use trait num_traits::ToPrimitive

    println!("solve {}", solve(a_f64, b_f64));

    println!("common solve {}", common_solve(a_f64, b_f64));
    println!("common solve <f64> {}", common_solve::<f64>(a_f64, b_f64));

    println!("super solve {}", super_solve(a_f32, b_f64));
    println!("super solve <f64, f64> {}", super_solve(a_f64, b_f64));
    println!("super solve <f32, f64> {}", super_solve(a_f32, b_f64));

    let a_i32: i32 = 3;
    println!("util solve <i32, f64> {}", util_solve(a_i32, b_f64));
}
