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
pub mod mutex;
pub mod mpsc;
pub mod fs;
pub mod serde;
mod default;

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
    // threadscoped::test();
    // mutex::test();
    // mutex::test_async();
    // mpsc::test();
    // mpsc::test_loop();

    // fs::test_dirs();
    // fs::test_create_delete_file();
    // fs::test_read_file();

    // serde::test();
    default::test();
}
