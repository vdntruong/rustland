pub mod closures;
pub mod rmatch;
pub mod optiontest;
pub mod traitgenerics;
pub mod vector;
pub mod hashmap;
pub mod hashset;
pub mod timetest;
pub mod thread;
pub mod threadscoped;

fn main() {
    // closures::test_closures();
    // rmatch::test();
    // rmatch::test_array();
    // optiontest::test();
    // traitgenerics::test();
    // vector::test();
    // hashmap::test();
    // hashset::test();
    // timetest::test();
    // timetest::test_chrono()
    // thread::spawn_thread();
    threadscoped::test();
}
