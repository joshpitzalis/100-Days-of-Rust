fn main() {
    println!("🧪 Error Handling Calculator");
}

// // ⛳️ Step 1 - Install dialoguer and create an input helper
// // `dialoguer` is a crate for building interactive CLI prompts.
// // Run: cargo add dialoguer
// use dialoguer::Input;

// fn main() {
//     println!("🧪 Error Handling Calculator");
//     let name: String = Input::new()
//         .with_prompt("What's your name?")
//         .interact_text()
//         .unwrap();
//     println!("Hello, {}!", name);
// }

// // ⛳️ Step 2 - Add a menu loop with Select
// // `Select` creates an interactive menu with arrow key navigation.
// // `loop` creates an infinite loop - we'll break out when user chooses exit.
// use dialoguer::Select;

// fn main() {
//     println!("🧪 Error Handling Calculator");

//     let options = vec!["Add", "Divide", "Exit"];

//     loop {
//         // `Select` returns the index of the chosen option
//         let selection = Select::new()
//             .with_prompt("Choose an option")
//             .items(&options)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => println!("You chose Add"),
//             1 => println!("You chose Divide"),
//             2 => {
//                 println!("👋 Exiting.");
//                 break;
//             }
//             _ => println!("❌ Invalid choice."),
//         }
//     }
// }

// // ⛳️ Step 3 - Parse numbers with Result and the ? operator
// // `Result<T, E>` is Rust's way of handling operations that can fail.
// // `ParseFloatError` is the error type returned when parsing fails.
// // The `?` operator is shorthand: if Ok, unwrap the value; if Err, return early.
// use dialoguer::{Input, Select};
// use std::num::ParseFloatError;

// fn main() {
//     println!("🧪 Error Handling Calculator");

//     let options = vec!["Add", "Divide", "Exit"];

//     loop {
//         let selection = Select::new()
//             .with_prompt("Choose an option")
//             .items(&options)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => {
//                 // `match` on Result to handle both Ok and Err cases
//                 match parse_two_numbers() {
//                     Ok((a, b)) => println!("✅ Result: {} + {} = {}", a, b, a + b),
//                     Err(e) => eprintln!("❌ Error: {}", e),
//                 }
//             }
//             1 => println!("You chose Divide"),
//             2 => {
//                 println!("👋 Exiting.");
//                 break;
//             }
//             _ => println!("❌ Invalid choice."),
//         }
//     }
// }

// /// Parse two numbers with error handling
// fn parse_two_numbers() -> Result<(f64, f64), ParseFloatError> {
//     // `Input::new()` creates a text prompt
//     // `.interact_text()` returns a String, which we then parse
//     // The `?` propagates errors up to the caller automatically
//     let a: String = Input::new()
//         .with_prompt("Enter first number")
//         .interact_text()
//         .unwrap();
//     let a: f64 = a.parse()?;

//     let b: f64 = Input::<String>::new()
//         .with_prompt("Enter second number")
//         .interact_text()
//         .expect("Failed to read second input")
//         .parse()?;

//     Ok((a, b))
// }

// // ⛳️ Step 4 - Add division with custom error handling
// // Sometimes you need custom error messages instead of standard error types.
// // `Result<f64, String>` uses String as the error type for flexibility.
// // This shows how to create your own error conditions (like divide by zero).
// use dialoguer::{Input, Select};
// use std::num::ParseFloatError;

// fn main() {
//     println!("🧪 Error Handling Calculator");

//     let options = vec!["Add", "Divide", "Exit"];

//     loop {
//         let selection = Select::new()
//             .with_prompt("Choose an option")
//             .items(&options)
//             .interact()
//             .unwrap();

//         match selection {
//             0 => match parse_two_numbers() {
//                 Ok((a, b)) => println!("✅ Result: {} + {} = {}", a, b, a + b),
//                 Err(e) => eprintln!("❌ Error: {}", e),
//             },
//             1 => match parse_two_numbers() {
//                 Ok((a, b)) => {
//                     // Nested match: first check parsing, then check division
//                     match divide(a, b) {
//                         Ok(result) => println!("✅ Result: {} / {} = {}", a, b, result),
//                         Err(e) => eprintln!("❌ Error: {}", e),
//                     }
//                 }
//                 Err(e) => eprintln!("❌ Error: {}", e),
//             },
//             2 => {
//                 println!("👋 Exiting.");
//                 break;
//             }
//             _ => println!("❌ Invalid choice."),
//         }
//     }
// }

// /// Parse two numbers with error handling
// fn parse_two_numbers() -> Result<(f64, f64), ParseFloatError> {
//     let a: String = Input::new()
//         .with_prompt("Enter first number")
//         .interact_text()
//         .unwrap();
//     let a: f64 = a.parse()?;

//     let b: String = Input::new()
//         .with_prompt("Enter second number")
//         .interact_text()
//         .unwrap();
//     let b: f64 = b.parse()?;

//     Ok((a, b))
// }

// /// Division with custom error handling
// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         // Return a custom error message as a String
//         Err("Cannot divide by zero.".to_string())
//     } else {
//         Ok(a / b)
//     }
// }

// - unwrap() => Quick prototyping, you're confident it won't fail
// - ? => You want to propagate the error up to the caller (no crash)
// - match => You want to handle both Ok and Err cases explicitly
// - expect("msg") => You want a clear error message if it panics
