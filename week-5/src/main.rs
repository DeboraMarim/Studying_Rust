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

// Task structure with lifetime annotation
struct Task<'a> {
    description: &'a str,
    priority: u8,
}

// Trait definition
trait Loggable {
    fn log(&self) -> String;
}

// Implementing the Loggable trait for the Task struct
impl<'a> Loggable for Task<'a> {
    fn log(&self) -> String {
        format!("Priority {}: {}", self.priority, self.description)
    }
}

fn main() {
    println!("Log Registration Program");

    println!("Enter the task description:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");

    println!("Enter the task priority:");
    let mut priority = String::new();
    io::stdin()
        .read_line(&mut priority)
        .expect("Failed to read input");

    let priority: u8 = priority.trim().parse().expect("Invalid priority");

    let task = Task {
        description: description.trim(),
        priority,
    };

    match register_log_message(&task.log()) {
        Ok(_) => println!("Task registered successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
