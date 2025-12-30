use std::fs;
use std::io::Error;

fn main() {
    let _ = divide(5.0, 2.0); // -> if syntax (type comparison) or pattern matching

    match divide(5.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    match validate_email("test@gmail.com".to_string()) {
        Ok(_) => println!("Email is valid"), // _ is a wildcard, it matches any value.
        // We can also use '..' to match any value.
        Err(error) => println!("Error: {}", error),
    }

    let content = fs::read_to_string("logs.txt");
    // println!("{:#?}", text);

    // let mut error_logs = vec![];
    match content {
        Ok(text) => {
            let error_logs = extract_errors(text.as_str());
            println!("{:#?}", error_logs);

            match fs::write("error_logs.txt", error_logs.join("\n")) {
                Ok(..) => println!("Error logs written successfully"),
                Err(error) => println!("Error: {}", error),
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    // Err(
    //     Os {
    //         code: 2,
    //         kind: NotFound,
    //         message: "No such file or directory",
    //     },
    // )
}

fn main_with_result() -> Result<(), Error> {
    let content = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(content.as_str());
    fs::write("error_logs.txt", error_logs.join("\n"))?;

    Ok(())
}

fn short_way() {
    // short way with expect methods, expect will panic if the result is an error
    let lines = fs::read_to_string("error_logs.txt").expect("Failed to read file");
    let errors = extract_errors(lines.as_str());
    let _ = fs::write("error_logs.txt", errors.join("\n")).expect("Failed to write file");
    // or
    // fs::write("error_logs.txt", errors.join("\n")).expect("Failed to write file");
}

fn extract_errors(text: &str) -> Vec<String> {
    // fn extract_errors(text: &str) -> Vec<&str>
    let slip_text = text.split("\n");
    let mut result = vec![];

    for line in slip_text {
        if line.starts_with("ERROR") {
            result.push(line.to_string());
            // result.push(line);
        }
    }

    result
}

// Empty tuple () is the convention for success
fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(()) // empty tuple is the convention for success
    } else {
        Err(Error::other("Email is invalid"))
    }
}

// Result enum is an enum that can be either Ok or Err (Success or Failure)
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
