use std::fs::File;
use std::io::{self, Write};

// Defining a custom error enum
#[derive(Debug)]
enum LogError {
    FileCreationError,
    WriteError,
}

// Implementing the std::error::Error trait for the custom error enum
impl std::error::Error for LogError {}

// Implementing the std::fmt::Display trait to provide user-friendly error messages
impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogError::FileCreationError => write!(f, "Error creating the log file"),
            LogError::WriteError => write!(f, "Error writing to the log file"),
        }
    }
}

// Function to register a log message
fn register_log_message(message: &str) -> Result<(), LogError> {
    let mut file = File::create("log.txt").map_err(|_| LogError::FileCreationError)?;

    file.write_all(message.as_bytes())
        .map_err(|_| LogError::WriteError)?;

    Ok(())
}

fn main() {
    println!("Log Registration Program");

    println!("Enter the log message:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match register_log_message(&input) {
        Ok(_) => println!("Log message registered successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
