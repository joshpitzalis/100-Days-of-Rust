fn main() {
    println!("Hello, world!");
}

// // ⛳️ Step 1 - Import dialoguer and define the State enum

// // In Rust, enums can hold different data in each variant:
// // - Unit variants: `Start`, `Complete` (no data)
// // - Tuple variants: `EnterEmail(String)` (unnamed field)
// // - Struct variants: `Confirm { name, email }` (named fields)

// // Run: cargo add dialoguer

// use dialoguer::{Confirm, Input};

// fn main() {
//     println!("🔄 State Machine: Signup Wizard");
// }

// /// State enum for signup process
// enum State {
//     Start,
//     EnterName,
//     EnterEmail(String),
//     Confirm { name: String, email: String },
//     Complete,
// }

// // ⛳️ Step 2 - Initialize state and create the main loop
// // State machines work by:
// // 1. Starting in an initial state
// // 2. Checking current state and taking action
// // 3. Transitioning to a new state
// // 4. Repeating until reaching a terminal state
// //
// // `let mut state` - We need `mut` because state changes over time.
// // `loop { ... }` - Infinite loop; we'll `break` when reaching Complete.
// //
// use dialoguer::{Confirm, Input};

// fn main() {
//     println!("🔄 State Machine: Signup Wizard");

//     let mut state = State::Start;

//     loop {
//         match state {
//             State::Start => {
//                 println!("Welcome! Let's begin your signup.");
//                 // Transition to next state
//                 state = State::EnterName;
//             }
//             State::Complete => {
//                 println!("🎉 Signup complete!");
//                 break;
//             }
//             // Temporary: skip other states for now
//             _ => {
//                 state = State::Complete;
//             }
//         }
//     }
// }

// enum State {
//     Start,
//     EnterName,
//     EnterEmail(String),
//     Confirm { name: String, email: String },
//     Complete,
// }

// // ⛳️ Step 3 - Handle EnterName state with dialoguer Input
// // `Input::<String>::new()` creates a new text input prompt.
// // `.with_prompt()` sets the prompt text shown to the user.
// // `.interact_text()` displays the prompt and waits for input.
// //
// // We validate input before transitioning:
// // - Empty name? Stay in EnterName state
// // - Valid name? Move to EnterEmail, carrying the name with us
// //
// // `State::EnterEmail(name)` - The tuple variant stores the name
// // so we can use it in the next state.
// use dialoguer::{Confirm, Input};

// fn main() {
//     println!("🔄 State Machine: Signup Wizard");

//     let mut state = State::Start;

//     loop {
//         match state {
//             State::Start => {
//                 println!("Welcome! Let's begin your signup.");
//                 state = State::EnterName;
//             }
//             State::EnterName => {
//                 let name: String = Input::new()
//                     .with_prompt("Enter your name")
//                     .interact_text()
//                     .unwrap();

//                 if name.is_empty() {
//                     println!("❌ Name cannot be empty.");
//                     // Stay in same state (no transition)
//                 } else {
//                     // Transition with data: pass name to next state
//                     state = State::EnterEmail(name);
//                 }
//             }
//             State::Complete => {
//                 println!("🎉 Signup complete!");
//                 break;
//             }
//             _ => {
//                 state = State::Complete;
//             }
//         }
//     }
// }

// enum State {
//     Start,
//     EnterName,
//     EnterEmail(String),
//     Confirm { name: String, email: String },
//     Complete,
// }

// // ⛳️ Step 4 - Handle EnterEmail state with destructuring
// // `State::EnterEmail(name)` destructures the tuple variant,
// // extracting the `name` String for use in this arm.
// //
// // Simple validation: check if email contains "@".
// // On success, we transition to Confirm with both name and email.
// //
// // Struct variant syntax: `State::Confirm { name, email }`
// // This creates a variant with named fields.
// use dialoguer::{Confirm, Input};

// fn main() {
//     println!("🔄 State Machine: Signup Wizard");

//     let mut state = State::Start;

//     loop {
//         match state {
//             State::Start => {
//                 println!("Welcome! Let's begin your signup.");
//                 state = State::EnterName;
//             }
//             State::EnterName => {
//                 let name: String = Input::new()
//                     .with_prompt("Enter your name")
//                     .interact_text()
//                     .unwrap();

//                 if name.is_empty() {
//                     println!("❌ Name cannot be empty.");
//                 } else {
//                     state = State::EnterEmail(name);
//                 }
//             }
//             // Destructure: extract `name` from the tuple variant
//             State::EnterEmail(name) => {
//                 let email: String = Input::new()
//                     .with_prompt("Enter your email")
//                     .interact_text()
//                     .unwrap();

//                 if email.contains("@") {
//                     // Transition with struct variant (named fields)
//                     state = State::Confirm { name, email };
//                 } else {
//                     println!("❌ Invalid email format.");
//                     // Stay in same state, keeping the name
//                     state = State::EnterEmail(name);
//                 }
//             }
//             State::Complete => {
//                 println!("🎉 Signup complete!");
//                 break;
//             }
//             _ => {
//                 state = State::Complete;
//             }
//         }
//     }
// }

// enum State {
//     Start,
//     EnterName,
//     EnterEmail(String),
//     Confirm { name: String, email: String },
//     Complete,
// }

// // ⛳️ Step 5 - Handle Confirm state with dialoguer Confirm
// // `State::Confirm { name, email }` destructures the struct variant.
// //
// // `Confirm::new()` creates a yes/no prompt that returns a boolean.
// // This is cleaner than parsing "yes"/"no" strings manually.
// //
// // Based on the user's choice:
// // - true (yes) → Complete
// // - false (no) → back to EnterName to start over
// use dialoguer::{Confirm, Input};

// fn main() {
//     println!("🔄 State Machine: Signup Wizard");

//     let mut state = State::Start;

//     loop {
//         match state {
//             State::Start => {
//                 println!("Welcome! Let's begin your signup.");
//                 state = State::EnterName;
//             }
//             State::EnterName => {
//                 let name: String = Input::new()
//                     .with_prompt("Enter your name")
//                     .interact_text()
//                     .unwrap();

//                 if name.is_empty() {
//                     println!("❌ Name cannot be empty.");
//                 } else {
//                     state = State::EnterEmail(name);
//                 }
//             }
//             State::EnterEmail(name) => {
//                 let email: String = Input::new()
//                     .with_prompt("Enter your email")
//                     .interact_text()
//                     .unwrap();

//                 if email.contains("@") {
//                     state = State::Confirm { name, email };
//                 } else {
//                     println!("❌ Invalid email format.");
//                     state = State::EnterEmail(name);
//                 }
//             }
//             // Destructure struct variant with named fields
//             State::Confirm { name, email } => {
//                 println!("✅ Confirm your info:");
//                 println!("   Name: {}", name);
//                 println!("   Email: {}", email);

//                 // Confirm returns bool: true for yes, false for no
//                 let confirmed = Confirm::new()
//                     .with_prompt("Is this correct?")
//                     .interact()
//                     .unwrap();

//                 state = if confirmed {
//                     State::Complete
//                 } else {
//                     State::EnterName
//                 };
//             }
//             State::Complete => {
//                 println!("🎉 Signup complete!");
//                 break;
//             }
//         }
//     }
// }

// /// State enum for signup process
// enum State {
//     Start,
//     EnterName,
//     EnterEmail(String),
//     Confirm { name: String, email: String },
//     Complete,
// }

// State machines with no boiler -> https://youtu.be/z-0-bbc80JM?si=H_sqM_J9dKPrRB9X
