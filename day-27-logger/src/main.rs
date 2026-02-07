// ⛳️ Step 1 - Make sure everything works
fn main() {
    println!("📝 Logger Utility (writes to log.txt)");
}

// // ⛳️ Step 2 - Add interactive menu with dialoguer
// //  Run: cargo add dialoguer

// // `dialoguer::Select` creates an arrow-key navigable menu
// // `.interact()` blocks until the user makes a selection
// // use dialoguer::Select;
// // A `loop` lets users log multiple messages before exiting
// // `break` exits the loop when user selects "Exit"
// use dialoguer::Select;

// fn main() {
//     println!("📝 Logger Utility (writes to log.txt)");

//     loop {
//         let options = vec!["INFO", "WARN", "ERROR", "Exit"];
//         let selection = Select::new()
//             .with_prompt("Select a log level")
//             .items(&options)
//             .default(0)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => println!("INFO selected"),
//             1 => println!("WARN selected"),
//             2 => println!("ERROR selected"),
//             3 => {
//                 println!("👋 Exiting logger.");
//                 break;
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// // ⛳️ Step 3 - Add text input for log messages
// // `dialoguer::Input` collects user text input
// // `.interact_text()` returns the entered string
// use dialoguer::{Input, Select};

// fn main() {
//     println!("📝 Logger Utility (writes to log.txt)");

//     loop {
//         let options = vec!["INFO", "WARN", "ERROR", "Exit"];
//         let selection = Select::new()
//             .with_prompt("Select a log level")
//             .items(&options)
//             .default(0)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => log_message("INFO"),
//             1 => log_message("WARN"),
//             2 => log_message("ERROR"),
//             3 => {
//                 println!("👋 Exiting logger.");
//                 break;
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// fn log_message(level: &str) {
//     let message: String = Input::new()
//         .with_prompt(format!("Enter {} message", level))
//         .interact_text()
//         .unwrap();

//     println!("Would log: [{}] {}", level, message);
// }

// // ⛳️ Step 4 - Add timestamps with chrono
// // Run: cargo add chrono
// // `Local::now()` gets the current system time
// // `.format()` converts the time to a custom string format
// use chrono::Local;
// use dialoguer::{Input, Select};

// fn main() {
//     println!("📝 Logger Utility (writes to log.txt)");

//     loop {
//         let options = vec!["INFO", "WARN", "ERROR", "Exit"];
//         let selection = Select::new()
//             .with_prompt("Select a log level")
//             .items(&options)
//             .default(0)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => log_message("INFO"),
//             1 => log_message("WARN"),
//             2 => log_message("ERROR"),
//             3 => {
//                 println!("👋 Exiting logger.");
//                 break;
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// fn log_message(level: &str) {
//     let message: String = Input::new()
//         .with_prompt(format!("Enter {} message", level))
//         .interact_text()
//         .unwrap();

//     // // Format: YYYY-MM-DD HH:MM:SS
//     let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
//     let entry = format!("[{}] {}: {}\n", timestamp, level, message);

//     println!("Would log: {}", entry.trim());
// }

// // ⛳️ Step 5 - Write logs to file with append mode
// // `OpenOptions::new()` creates a file with specific permissions
// // `.create(true)` creates the file if it doesn't exist
// // `.append(true)` adds to the end without overwriting
// use chrono::Local;
// use dialoguer::{Input, Select};
// use std::fs::OpenOptions;
// use std::io::Write;

// fn main() {
//     println!("📝 Logger Utility (writes to log.txt)");

//     loop {
//         let options = vec!["INFO", "WARN", "ERROR", "Exit"];
//         let selection = Select::new()
//             .with_prompt("Select a log level")
//             .items(&options)
//             .default(0)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => log_message("INFO"),
//             1 => log_message("WARN"),
//             2 => log_message("ERROR"),
//             3 => {
//                 println!("👋 Exiting logger.");
//                 break;
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// fn log_message(level: &str) {
//     let message: String = Input::new()
//         .with_prompt(format!("Enter {} message", level))
//         .interact_text()
//         .unwrap();

//     let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
//     let entry = format!("[{}] {}: {}\n", timestamp, level, message);

//     // // Open file in append mode
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("log.txt")
//         .expect("❌ Unable to open log file");

//     // // Write the log entry
//     file.write_all(entry.as_bytes()).expect("❌ Write failed");

//     println!("✅ Logged successfully.");
// }

// // ⛳️ Step 6 - Add miette for rich error handling
// // Run: cargo add miette --features fancy
// // `miette::Result` is like `std::Result` but with better error messages
// // `.into_diagnostic()` converts any error into a miette diagnostic
// // `.wrap_err()` adds context to help users understand what failed
// use chrono::Local;
// use dialoguer::{Input, Select};
// use miette::{IntoDiagnostic, Result, WrapErr};
// use std::fs::OpenOptions;
// use std::io::Write;

// fn main() -> Result<()> {
//     println!("📝 Logger Utility (writes to log.txt)");

//     loop {
//         let options = vec!["INFO", "WARN", "ERROR", "Exit"];
//         let selection = Select::new()
//             .with_prompt("Select a log level")
//             .items(&options)
//             .default(0)
//             .interact()
//             .into_diagnostic()
//             .wrap_err("Failed to get user selection")?;

//         match selection {
//             0 => log_message("INFO")?,
//             1 => log_message("WARN")?,
//             2 => log_message("ERROR")?,
//             3 => {
//                 println!("👋 Exiting logger.");
//                 break;
//             }
//             _ => unreachable!(),
//         }
//     }

//     Ok(())
// }

// fn log_message(level: &str) -> Result<()> {
//     let message: String = Input::new()
//         .with_prompt(format!("Enter {} message", level))
//         .interact_text()
//         .into_diagnostic()
//         .wrap_err("Failed to read message input")?;

//     let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
//     let entry = format!("[{}] {}: {}\n", timestamp, level, message);

//     let mut file = OpenOptions::new()
//         // to test the error switch create() to false (after you have created the file by running the program once), then mess up the path in open() so that it tries to update a file that doesn't exist
//         .create(true)
//         .append(true)
//         .open("log.txt")
//         .into_diagnostic()
//         .wrap_err("Unable to open log file")?;

//     file.write_all(entry.as_bytes())
//         .into_diagnostic()
//         .wrap_err("Failed to write to log file")?;

//     println!("✅ Logged successfully.");
//     Ok(())
// }

// // // ⛳️ Step 7 - Add unit tests for error handling
// // // `#[cfg(test)]` marks this module as test-only code

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_log_file_creation() -> Result<()> {
//         // // Clean up any existing test log
//         let _ = fs::remove_file("test_log.txt");

//         // // Create a test log entry
//         let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
//         let entry = format!("[{}] INFO: Test message\n", timestamp);

//         let mut file = OpenOptions::new()
//             .create(true)
//             .append(true)
//             .open("test_log.txt")
//             .into_diagnostic()
//             .wrap_err("Unable to open test log file")?;

//         file.write_all(entry.as_bytes())
//             .into_diagnostic()
//             .wrap_err("Failed to write to test log file")?;

//         // // Verify file exists and contains content
//         let contents = fs::read_to_string("test_log.txt")
//             .into_diagnostic()
//             .wrap_err("Failed to read test log file")?;

//         assert!(contents.contains("INFO: Test message"));

//         // // Cleanup
//         // fs::remove_file("test_log.txt").into_diagnostic()?;
//         Ok(())
//     }

//     #[test]
//     fn test_invalid_file_path() {
//         // // Test that opening a file in an invalid directory produces an error
//         let result = OpenOptions::new()
//             .create(true)
//             .append(true)
//             .open("/invalid/path/log.txt")
//             .into_diagnostic()
//             .wrap_err("Unable to open log file");

//         assert!(result.is_err());
//     }
// }
