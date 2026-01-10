/*

https://docs.rs/csv/latest/csv/

Installation:
cargo add csv
 */

use csv::{Reader, ReaderBuilder};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    vin: String,
}

pub fn test() {
    let filename = "data.csv";

    match Reader::from_path(filename) {
        Ok(mut reader) => {
            for records in reader.records() {
                let record = records.unwrap();
                println!(
                    "{} {} {}",
                    record.get(0).unwrap(),
                    record.get(1).unwrap(),
                    record.get(2).unwrap()
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(9);
        }
    }

    _ = read_with_builder();
    _ = reader_with_serde();
}

fn read_with_builder() -> Result<(), Box<dyn Error>> {
    let filename = "data.csv";

    let mut rdr = ReaderBuilder::new()
        .double_quote(false)
        .comment(Some(b'#'))
        .has_headers(false)
        .from_path(filename)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn reader_with_serde() -> Result<(), Box<dyn Error>> {
    let filename = "data.csv";

    let mut rdr = ReaderBuilder::new()
        .double_quote(false)
        .comment(Some(b'#'))
        .has_headers(false)
        .delimiter(b':') // change this to ','
        .from_path(filename)?;

    for result in rdr.deserialize::<Vehicle>() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
