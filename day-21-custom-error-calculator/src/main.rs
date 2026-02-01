fn main() {
    println!("🧮 Custom Error Handling: Square Root Calculator");
}

// // ⛳️ Step 1 - Add user input with dialoguer
// // Run: cargo add dialoguer
// //
// // `dialoguer` is a crate for building interactive CLI prompts.
// // `Input::<String>::new()` creates a text input prompt builder.
// // `.with_prompt()` sets the message shown to the user.
// // `.interact_text()` displays the prompt and waits for input.
// use dialoguer::Input;

// fn main() {
//     println!("🧮 Custom Error Handling: Square Root Calculator");

//     // `Input::<String>` specifies we want a String back from the user
//     let input: String = Input::new()
//         .with_prompt("Enter a number")
//         .interact_text()
//         // `.expect()` panics with a message if the Result is Err
//         .expect("Failed to read input");

//     println!("You entered: {}", input);
// }

// // ⛳️ Step 2 - Parse input and calculate square root
// // `.parse::<f64>()` converts a string to a floating-point number.
// // It returns `Result<f64, ParseFloatError>` since parsing can fail.
// // `match` lets us handle both `Ok` (success) and `Err` (failure) cases.
// use dialoguer::Input;

// fn main() {
//     println!("🧮 Custom Error Handling: Square Root Calculator");

//     let input: String = Input::new()
//         .with_prompt("Enter a number")
//         .interact_text()
//         .expect("Failed to read input");

//     // `parse::<f64>()` attempts to convert the string to a 64-bit float
//     match input.trim().parse::<f64>() {
//         Ok(num) => {
//             // `.sqrt()` calculates the square root
//             let result = num.sqrt();
//             println!("✅ Square root: {}", result);
//         }
//         // `_` ignores the actual error value since we just print a generic message
//         Err(_) => eprintln!("❌ Invalid number format."),
//     }
// }

// // ⛳️ Step 3 - Define a custom error type
// // In Rust, you can create your own error types using enums.
// // `#[derive(Debug)]` lets us print the error for debugging.

// // `#[derive(Debug)]` automatically implements the Debug trait for printing
// #[derive(Debug)]
// enum MathError {
//     // Each enum variant represents a different kind of error.
//     // This variant represents the error case of a negative input
//     NegativeInput,
// }

// // ⛳️ Step 4 - Implement Display for user-friendly error messages
// // `std::fmt::Display` is the trait for user-facing output (like `ToString`).
// // When you `println!("{}", error)`, Rust calls your `Display` implementation.
// // `fmt::Formatter` is the output buffer where you write your message.

// use std::fmt;

// #[derive(Debug)]
// enum MathError {
//     NegativeInput,
// }

// // `impl fmt::Display for MathError` defines how the error is shown to users
// impl fmt::Display for MathError {
//     // `fmt` is called whenever the error is printed with `{}` format specifier
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // `match *self` checks which variant we have and writes the appropriate message
//         match *self {
//             MathError::NegativeInput => {
//                 write!(f, "Cannot calculate the square root of a negative number.")
//             }
//         }
//     }
// }

// // ⛳️ Step 5 - Implement the Error trait
// // `std::error::Error` is Rust's standard trait for error types.
// // Implementing it lets your error work with `?` operator and error-handling libraries.
// // The empty `impl` uses default implementations (we already have Display).
// use dialoguer::Input;
// use std::error::Error;
// use std::fmt;

// fn main() {
//     println!("🧮 Custom Error Handling: Square Root Calculator");

//     let input: String = Input::new()
//         .with_prompt("Enter a number")
//         .interact_text()
//         .expect("Failed to read input");

//     match input.trim().parse::<f64>() {
//         Ok(num) => {
//             if num < 0.0 {
//                 eprintln!("❌ {}", MathError::NegativeInput);
//             } else {
//                 let result = num.sqrt();
//                 println!("✅ Square root: {}", result);
//             }
//         }
//         Err(_) => eprintln!("❌ Invalid number format."),
//     }
// }

// #[derive(Debug)]
// enum MathError {
//     NegativeInput,
// }

// impl fmt::Display for MathError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             MathError::NegativeInput => {
//                 write!(f, "Cannot calculate the square root of a negative number.")
//             }
//         }
//     }
// }

// // This empty impl tells Rust that MathError satisfies the Error trait
// // The Error trait requires Debug + Display, which we already have
// impl Error for MathError {}

// // ⛳️ Step 6 - Create a function that returns Result with our custom error
// // `Result<T, E>` is Rust's way of returning either success (Ok) or failure (Err).
// // Here we return `Result<f64, MathError>` - either a valid result or our custom error.
// // This pattern lets callers decide how to handle errors.
// use dialoguer::Input;
// use std::error::Error;
// use std::fmt;

// fn main() {
//     println!("🧮 Custom Error Handling: Square Root Calculator");

//     let input: String = Input::new()
//         .with_prompt("Enter a number")
//         .interact_text()
//         .expect("Failed to read input");

//     match input.trim().parse::<f64>() {
//         Ok(num) => {
//             // Now we use our function that returns Result
//             match calculate_sqrt(num) {
//                 Ok(result) => println!("✅ Square root: {}", result),
//                 // Our Display impl provides the error message
//                 Err(e) => eprintln!("❌ {}", e),
//             }
//         }
//         Err(_) => eprintln!("❌ Invalid number format."),
//     }
// }

// #[derive(Debug)]
// enum MathError {
//     NegativeInput,
// }

// impl fmt::Display for MathError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             MathError::NegativeInput => {
//                 write!(f, "Cannot calculate the square root of a negative number.")
//             }
//         }
//     }
// }

// impl Error for MathError {}

// // This function returns `Result<f64, MathError>`:
// // - `Ok(f64)` when the input is valid (non-negative)
// // - `Err(MathError)` when the input is invalid (negative)
// fn calculate_sqrt(x: f64) -> Result<f64, MathError> {
//     if x < 0.0 {
//         // Return an error for negative inputs
//         Err(MathError::NegativeInput)
//     } else {
//         // Return the successful result wrapped in Ok
//         Ok(x.sqrt())
//     }
// }
