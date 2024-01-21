// Using Err, Ok for error handler

// Example
//#[derive(Debug)]
//struct DivisionByZeroError;
//
//fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
//    if divisor == 0.0 {
//        Err(DivisionByZeroError)
//    } else {
//        Ok(dividend / divisor)
//    }
//}
//
//fn main() {
//    println!("{:?}", safe_division(88989.0, 3.0));
//    println!("{:?}", safe_division(4.0, 0.0));
//    println!("{:?}", safe_division(0.0, 2.0));
//}

// Exercise
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Access a file at a specified path
    // ---------------------------------
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error    
    let mut file: File = match File::open("../eje.txt") {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    // Read file contents into `String` variable with `read_to_string`
    // ---------------------------------
    // Success path is already filled in
    // Return from the function early if it is an error
    match file.read_to_string(&mut string) {
        Ok(_) => {
            for line in string.lines() {
                println!("{}", line);
            }
            Ok(string)
        }
        Err(io_error) => return Err(io_error)
    }
}

// to review, printing two times the txt
fn main() {
    if read_file_contents(PathBuf::from("src/error_handler.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}