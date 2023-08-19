use std::fs::File;   // Import the File struct from the standard library
use std::io::{self, Read};   // Import IO-related modules, including error handling

// Define a function to read file contents
fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;   // Try to open the file, ? handles errors
    let mut contents = String::new();   // Create a mutable string to store file contents
    file.read_to_string(&mut contents)?;   // Read file contents into the string, ? handles errors
    Ok(contents)   // Return the contents as a Result
}

// Define a function to perform division
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Division by zero is not allowed"));   // Return an error if division by zero is attempted
    }
    Ok(a / b)   // Return the division result as a Result
}

fn main() {
    // Example of using the Result type and the ? operator to read file contents
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),   // Print the contents if reading succeeds
        Err(error) => println!("Error reading file: {}", error),   // Print an error message if reading fails
    }

    // Example of causing a panic by accessing an index out of bounds
    let vec = vec![1, 2, 3];
    println!("Item at index 10: {}", vec.get(10).unwrap()); // This will panic if the index is out of bounds

    // Example of using the divide function
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result of division: {}", result),   // Print the result if division succeeds
        Err(error) => println!("Error: {}", error),   // Print an error message if division fails
    }
}
