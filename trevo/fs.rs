/*
https://doc.rust-lang.org/std/fs/index.html
 */
use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn test_dirs() {
    // create dir
    let str_path = "./data";
    let path = Path::new(str_path);

    if path.exists() {
        println!("{} exists", path.display());
        return
    }

    println!("{} does not exists", path.display());
    match fs::create_dir(path) {
        Ok(_) => {
            println!("Dir created: {:?}", fs::canonicalize(path).unwrap());
        }
        Err(e) => {
            println!("Dir create error: {:?}", e);
        }
    };

    // sleep(Duration::from_secs(3));

    // remove empty dir
    // match fs::remove_dir(path) {
    //     Ok(_) => {
    //         println!("Empty Dir removed: {:?}", path.display());
    //     }
    //     Err(e) => {
    //         println!("Empty Dir remove error: {:?}", e);
    //     }
    // }

    // remove dir and data in it
    // match fs::remove_dir_all(path) {
    //     Ok(_) => {
    //         println!("Data-Dir removed: {:?}", path.display());
    //     }
    //     Err(e) => {
    //         println!("Data-Dir remove error: {:?}", e);
    //     }
    // }
}

pub fn test_create_delete_file() {
    // create file
    let path = "./data/myfile.txt";
    let contents = "Something went wrong reading the file";

    match fs::write(path, contents) {
        Ok(_) => {
            println!("File written to {:?}", path);
        }
        Err(e) => {
            println!("File write error: {:?}", e);
        }
    }

    // remove a file
    // _ = fs::remove_file(path)
}

pub fn test_read_file() {
    let path = "./data/myfile.txt";
    let convert_byte_to_str = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        a
    };
    match fs::read(path) {
        Ok(contents) => {
            // ASIC code
            println!("File contents: {:?}", contents);
            println!("Str content: {:#?}", contents.iter().fold("".to_string(), convert_byte_to_str));
        }
        Err(e) => {
            println!("File read error: {:?}", e);
        }
    }
}