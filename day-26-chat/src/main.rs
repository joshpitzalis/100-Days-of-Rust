// fn main() {
//     println!("Hello, world!");
// }

// // ⛳️ Step 1 - Read user input with dialoguer
// // Run: cargo add dialoguer

// use dialoguer::Input;

// fn main() {
//     // // The turbofish `::<String>` tells Input what type to return.
//     // // dialoguer can also parse into numbers, paths, etc.
//     let input = Input::<String>::new()
//         .with_prompt("You")
//         .interact_text()
//         .unwrap();

//     println!("You typed: {}", input);
// }

// // ⛳️ Step 2 - Add an input loop with an exit command
// // `loop {}` creates an infinite loop — perfect for REPLs and chat interfaces.
// // We use `.to_lowercase()` so "EXIT", "Exit", and "exit" all work.
// // `break` exits the loop when the user types "exit".
// use dialoguer::Input;

// fn main() {
//     println!("💬 ChatBot CLI - Type 'exit' to quit");

//     loop {
//         let input: String = Input::<String>::new()
//             .with_prompt("You")
//             .interact_text()
//             .unwrap();

//         // // `.to_lowercase()` returns a new String — it doesn't modify `input`
//         if input.to_lowercase() == "exit" {
//             println!("👋 Goodbye!");
//             break;
//         }

//         println!("Echo: {}", input);
//     }
// }

// // ⛳️ Step 3 - Add bot reply logic with pattern matching
// // `bot_reply()` takes a `&str` reference (borrowed, not owned) and returns an owned `String`.
// // `.contains()` checks if a substring exists — simple but effective for keyword matching.
// // Each branch returns a `String` using `.to_string()` to convert from a `&str` literal.
// // Rust's `if/else if/else` chain works like a match here — first true branch wins.
// use dialoguer::Input;

// fn main() {
//     println!("💬 ChatBot CLI - Type 'exit' to quit");

//     loop {
//         let input: String = Input::<String>::new()
//             .with_prompt("You")
//             .interact_text()
//             .unwrap();

//         if input.to_lowercase() == "exit" {
//             println!("👋 Goodbye!");
//             break;
//         }

//         // // `&input` borrows the String as a &str — no ownership transfer needed
//         let response = bot_reply(&input);
//         println!("Bot: {}", response);
//     }
// }

// fn bot_reply(message: &str) -> String {
//     // // Convert to lowercase once so all comparisons are case-insensitive
//     let msg = message.to_lowercase();

//     if msg.contains("hello") {
//         "Hi there!".to_string()
//     } else if msg.contains("how are you") {
//         "I'm just code, but I'm doing fine!".to_string()
//     } else if msg.contains("rust") {
//         "Rust is memory-safe and fearless!".to_string()
//     } else {
//         "I don't understand that yet.".to_string()
//     }
// }

// // ⛳️ Step 4 - Track chat history with Vec<String> and add ColorfulTheme
// // Run: cargo add console

// // `ColorfulTheme::default()` gives dialoguer styled prompts with color and formatting.
// // `Input::with_theme(&theme)` passes a reference to the theme — the `&` means we
// // borrow it rather than moving ownership, so we can reuse it every loop iteration.
// use dialoguer::{Input, theme::ColorfulTheme};

// fn main() {
//     // // `ColorfulTheme` lives in `dialoguer::theme` — it adds colors to the prompt
//     let theme = ColorfulTheme::default();

//     println!("💬 ChatBot CLI - Type 'exit' to quit");

//     // // `mut` because we'll push messages into it on every loop iteration
//     let mut history: Vec<String> = Vec::new();

//     loop {
//         // // `&theme` borrows the theme — we don't want to give ownership away
//         // // since we need it again on the next loop iteration
//         let input: String = Input::with_theme(&theme)
//             .with_prompt("You")
//             .interact_text()
//             .unwrap();

//         if input.to_lowercase() == "exit" {
//             println!("👋 Goodbye!");
//             break;
//         }

//         // // `format!` works like `println!` but returns a String instead of printing
//         history.push(format!("You: {}", input));

//         let response = bot_reply(&input);
//         println!("Bot: {}", response);
//         history.push(format!("Bot: {}", response));
//     }

//     // // Print the full conversation log after the user exits
//     println!("\n🗒️ Chat History:");
//     for line in &history {
//         println!("{}", line);
//     }
// }

// fn bot_reply(message: &str) -> String {
//     let msg = message.to_lowercase();

//     if msg.contains("hello") {
//         "Hi there!".to_string()
//     } else if msg.contains("how are you") {
//         "I'm just code, but I'm doing fine!".to_string()
//     } else if msg.contains("rust") {
//         "Rust is memory-safe and fearless!".to_string()
//     } else {
//         "I don't understand that yet.".to_string()
//     }
// }

// // ⛳️ Step 5 - Add input history with arrow keys and Select for mode picking
// // `BasicHistory` lets users press Up/Down arrows to recall previous inputs.
// // It requires the `history` feature: `cargo add dialoguer --features history`
// //
// // `Select` renders a menu the user navigates with arrow keys.
// // `.items(&["Option A", "Option B"])` takes a slice reference `&[&str]`.
// // `.default(0)` pre-selects the first item.
// // `.interact()` returns a `usize` index of the selected item.
// //
// // `match` is Rust's pattern matching — like a switch statement but exhaustive.
// // The compiler ensures every possible value is handled.
// use dialoguer::{BasicHistory, Input, Select, theme::ColorfulTheme};

// fn main() {
//     let theme = ColorfulTheme::default();

//     // // `Select` renders an interactive menu — arrow keys to navigate, Enter to select.
//     // // `.items()` takes a slice of display strings.
//     // // `.interact().unwrap()` returns the index (usize) of the chosen item.
//     let modes = &["🤖 Local Bot (offline)", "🌐 OpenAI API (coming soon)"];
//     let mode = Select::with_theme(&theme)
//         .with_prompt("Choose chat mode")
//         .items(modes)
//         .default(0)
//         .interact()
//         .unwrap();

//     // // `match` on the index — Rust requires all cases to be handled
//     match mode {
//         0 => println!("Starting local bot..."),
//         1 => println!("OpenAI mode will be added in Step 7!"),
//         // // `_` is the catch-all pattern — needed because usize has many possible values
//         // // `unreachable!()` panics if hit — we know Select only returns 0 or 1
//         _ => unreachable!(),
//     }

//     println!("💬 ChatBot CLI - Type 'exit' to quit");
//     println!("💡 Use Up/Down arrows to recall previous messages\n");

//     let mut history: Vec<String> = Vec::new();
//     // // `BasicHistory` tracks inputs for arrow-key recall.
//     // // `.max_entries(20)` limits memory usage.
//     // // `.no_duplicates(true)` skips repeated entries.
//     let mut input_history = BasicHistory::new().max_entries(20).no_duplicates(true);

//     loop {
//         // // `.history_with(&mut input_history)` enables Up/Down arrow recall.
//         // // `&mut` is a mutable borrow — dialoguer needs to write new entries into it.
//         let input: String = Input::with_theme(&theme)
//             .with_prompt("You")
//             .history_with(&mut input_history)
//             .interact_text()
//             .unwrap();

//         if input.to_lowercase() == "exit" {
//             println!("👋 Goodbye!");
//             break;
//         }

//         history.push(format!("You: {}", input));

//         let response = bot_reply(&input);
//         println!("Bot: {}", response);
//         history.push(format!("Bot: {}", response));
//     }

//     println!("\n🗒️ Chat History:");
//     for line in &history {
//         println!("{}", line);
//     }
// }

// fn bot_reply(message: &str) -> String {
//     let msg = message.to_lowercase();

//     if msg.contains("hello") {
//         "Hi there!".to_string()
//     } else if msg.contains("how are you") {
//         "I'm just code, but I'm doing fine!".to_string()
//     } else if msg.contains("rust") {
//         "Rust is memory-safe and fearless!".to_string()
//     } else {
//         "I don't understand that yet.".to_string()
//     }
// }

// // ⛳️ Step 6 - Connect to OpenAI with reqwest, tokio, and serde
// // Now we wire up the Select menu so mode 1 calls the real OpenAI API.
// // Run: cargo add reqwest -F reqwest/json tokio -F tokio/full serde -F serde/derive serde_json dotenv

// //
// // **New concepts introduced:**
// // `#[tokio::main]` — An attribute that sets up the async runtime. Rust has async/await
// // syntax built in, but needs an external runtime (like tokio) to actually execute futures.
// // `async fn` — Declares a function that returns a Future. You `.await` it to get the result.
// // `Result<T, E>` — Rust's standard error handling type. `Ok(value)` or `Err(error)`.
// // The `?` operator — Shorthand: if the Result is Err, return early with that error.
// // `#[derive(Serialize)]` / `#[derive(Deserialize)]` — serde macros that auto-generate
// // code to convert structs to/from JSON. No manual parsing needed.
// // `.clone()` — Creates a deep copy. Needed here because we reuse `messages` after sending.
// // `&mut` — A mutable reference. Lets `ask_openai` modify the conversation without taking ownership.
// // `enum` — Rust's algebraic data type. Here we use it to represent the chat mode cleanly.
// //
// // Create a `.env` file in your project root with: OPENAI_API_KEY=your_key_here
// use dialoguer::{BasicHistory, Confirm, Input, Select, theme::ColorfulTheme};
// use dotenv::dotenv;
// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use std::env;

// // // These structs define the shape of data we send to and receive from OpenAI.
// // // `#[derive(Serialize)]` lets serde convert a struct → JSON for the request body.
// // // `#[derive(Deserialize)]` lets serde convert JSON → struct for the response.
// #[derive(Serialize)]
// struct ChatRequest {
//     model: String,
//     messages: Vec<Message>,
// }

// // // `Clone` is needed so we can `.clone()` the messages vec when building the request
// #[derive(Serialize, Deserialize, Clone)]
// struct Message {
//     role: String,
//     content: String,
// }

// #[derive(Deserialize)]
// struct ChatResponse {
//     choices: Vec<ChatChoice>,
// }

// #[derive(Deserialize)]
// struct ChatChoice {
//     message: ChatMessage,
// }

// #[derive(Deserialize)]
// struct ChatMessage {
//     content: String,
// }

// // // `enum` defines a type with a fixed set of variants — like a tagged union.
// // // Much safer than using raw integers or strings to represent modes.
// enum ChatMode {
//     Local,
//     OpenAI,
// }

// // // `#[tokio::main]` transforms main into an async function with a tokio runtime.
// // // Without this, you can't use `.await` in main.
// // // `Result<(), String>` means: return nothing on success, or a String error message.
// #[tokio::main]
// async fn main() -> Result<(), String> {
//     // // Load environment variables from .env file (where your API key lives)
//     dotenv().ok();

//     let theme = ColorfulTheme::default();

//     let modes = &["🤖 Local Bot (offline)", "🌐 OpenAI API"];
//     let mode_index = Select::with_theme(&theme)
//         .with_prompt("Choose chat mode")
//         .items(modes)
//         .default(0)
//         .interact()
//         .unwrap();

//     // // `match` converts the index into our enum — type-safe mode tracking
//     let chat_mode = match mode_index {
//         0 => ChatMode::Local,
//         1 => ChatMode::OpenAI,
//         _ => unreachable!(),
//     };

//     // // The HTTP client is only needed for OpenAI mode, but creating it is cheap
//     let client = Client::new();
//     // // This Vec<Message> holds the full conversation in the format OpenAI expects
//     let mut messages: Vec<Message> = Vec::new();
//     let mut display_history: Vec<String> = Vec::new();

//     // // `matches!()` is a macro that returns true if the value matches the pattern
//     if matches!(chat_mode, ChatMode::OpenAI) {
//         // // The system message sets the AI's personality and instructions.
//         // // It's invisible to the user but shapes every response.
//         messages.push(Message {
//             role: "system".to_string(),
//             content: "You are a helpful and friendly assistant. Keep responses concise."
//                 .to_string(),
//         });
//     }

//     println!("💬 ChatBot CLI - Type 'exit' to quit");
//     println!("💡 Use Up/Down arrows to recall previous messages\n");

//     let mut input_history = BasicHistory::new().max_entries(20).no_duplicates(true);

//     loop {
//         let input: String = Input::with_theme(&theme)
//             .with_prompt("You")
//             .history_with(&mut input_history)
//             .allow_empty(false)
//             .interact_text()
//             .unwrap();

//         if input.to_lowercase() == "exit" {
//             //  `Confirm::new()` renders a yes/no prompt — returns a `bool`.
//             // `.default(false)` means pressing Enter without typing defaults to "no".
//             //
//             // `.allow_empty(false)` on Input prevents submitting blank messages.
//             // This is dialoguer's built-in validation — no manual checking needed.
//             let confirmed = Confirm::with_theme(&theme)
//                 .with_prompt("Do you really want to quit?")
//                 .default(false)
//                 .interact()
//                 .unwrap();

//             if confirmed {
//                 println!("👋 Goodbye!");
//                 break;
//             }
//             continue;
//         }

//         display_history.push(format!("You: {}", input));

//         // // `match` on the enum — Rust ensures we handle both variants
//         let response = match &chat_mode {
//             ChatMode::Local => bot_reply(&input),
//             ChatMode::OpenAI => {
//                 // // Push user message into the API conversation
//                 messages.push(Message {
//                     role: "user".to_string(),
//                     content: input.to_string(),
//                 });
//                 // // `.await` pauses until the HTTP request completes
//                 // // `?` propagates any error up to main
//                 ask_openai(&client, &mut messages).await?
//             }
//         };

//         let label = match &chat_mode {
//             ChatMode::Local => "Bot",
//             ChatMode::OpenAI => "AI",
//         };
//         println!("{}: {}", label, response);
//         display_history.push(format!("{}: {}", label, response));
//     }

//     println!("\n🗒️ Chat History:");
//     for line in &display_history {
//         println!("{}", line);
//     }

//     Ok(())
// }

// // // `&Client` — immutable borrow of the HTTP client (shared, not consumed).
// // // `&mut Vec<Message>` — mutable borrow so we can push the assistant's reply.
// // // Returns `Result<String, String>` — the AI's reply text, or an error message.
// async fn ask_openai(client: &Client, messages: &mut Vec<Message>) -> Result<String, String> {
//     let request_body = ChatRequest {
//         model: "gpt-4o-mini".to_string(),
//         // // `.clone()` makes a deep copy of messages for serialization
//         messages: messages.clone(),
//     };

//     // // Build and send the HTTP POST request. Each method chains onto the builder.
//     // // `env::var("OPENAI_API_KEY")` reads from environment variables (loaded by dotenv).
//     // // `.expect()` panics with a message if the env var is missing — fine for a CLI tool.
//     let response = client
//         .post("https://api.openai.com/v1/chat/completions")
//         .header("Content-Type", "application/json")
//         .header(
//             "Authorization",
//             format!(
//                 "Bearer {}",
//                 env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env")
//             ),
//         )
//         // // `.json(&request_body)` serializes our struct to JSON using serde
//         .json(&request_body)
//         .send()
//         .await
//         // // `.map_err(|e| e.to_string())` converts reqwest::Error → String
//         .map_err(|e| e.to_string())?;

//     // // The turbofish `::<ChatResponse>` tells Rust which type to deserialize into
//     let body = response
//         .json::<ChatResponse>()
//         .await
//         .map_err(|e| e.to_string())?;

//     // // `if let Some(choice)` — Pattern matching to safely unwrap the Option
//     if let Some(choice) = body.choices.last() {
//         // // Push the assistant's reply so the AI has full context on the next turn
//         messages.push(Message {
//             role: "assistant".to_string(),
//             content: choice.message.content.clone(),
//         });
//         Ok(choice.message.content.clone())
//     } else {
//         Err("No response from AI".to_string())
//     }
// }

// fn bot_reply(message: &str) -> String {
//     let msg = message.to_lowercase();

//     if msg.contains("hello") {
//         "Hi there!".to_string()
//     } else if msg.contains("how are you") {
//         "I'm just code, but I'm doing fine!".to_string()
//     } else if msg.contains("rust") {
//         "Rust is memory-safe and fearless!".to_string()
//     } else {
//         "I don't understand that yet.".to_string()
//     }
// }

// // ⛳️ Step 7 - Replace string errors with custom error types using thiserror
// // Run: cargo add thiserror
// //
// // In Step 6, we used `Result<String, String>` — errors were just plain strings.
// // That works, but you can't match on what went wrong or add structured context.
// //
// // `thiserror` generates `Error` trait implementations from enums.
// // Each variant represents a specific failure mode in your application.
// // `#[error("...")]` sets the Display message for each variant.
// // `#[from]` auto-generates a `From` impl, so `?` automatically converts
// // the source error into your custom type — no manual `.map_err()` needed.
// //
// // **Why custom errors matter:**
// // - `String` errors lose type information — you can't pattern match on them
// // - Custom enums let callers decide how to handle each failure mode
// // - `#[from]` eliminates boilerplate `.map_err()` calls throughout your code
// // - The compiler ensures you handle every variant when you `match`
// use dialoguer::{BasicHistory, Confirm, Input, Select, theme::ColorfulTheme};
// use dotenv::dotenv;
// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use std::env;
// use thiserror::Error;

// // // Each variant wraps a specific error type that can occur in our chat app.
// // // `#[error("...")]` defines what gets printed when the error is displayed.
// // // `#[from]` means `?` on a reqwest::Error auto-converts to ChatError::Http.
// #[derive(Error, Debug)]
// enum ChatError {
//     // // `#[from]` generates: impl From<reqwest::Error> for ChatError
//     // // So any `reqwest_call()?` automatically becomes ChatError::Http
//     #[error("HTTP request failed: {0}")]
//     Http(#[from] reqwest::Error),

//     // // For env var errors — missing API key, etc.
//     #[error("environment variable error: {0}")]
//     EnvVar(#[from] env::VarError),

//     // // For errors that don't map to a library type — like "no response from AI"
//     // // No `#[from]` here since we construct this variant manually
//     #[error("{0}")]
//     Api(String),
// }

// #[derive(Serialize)]
// struct ChatRequest {
//     model: String,
//     messages: Vec<Message>,
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct Message {
//     role: String,
//     content: String,
// }

// #[derive(Deserialize)]
// struct ChatResponse {
//     choices: Vec<ChatChoice>,
// }

// #[derive(Deserialize)]
// struct ChatChoice {
//     message: ChatMessage,
// }

// #[derive(Deserialize)]
// struct ChatMessage {
//     content: String,
// }

// enum ChatMode {
//     Local,
//     OpenAI,
// }

// // // Now main returns our custom ChatError instead of a plain String.
// // // Any `?` in the function body will auto-convert via #[from] impls.
// #[tokio::main]
// async fn main() -> Result<(), ChatError> {
//     dotenv().ok();

//     let theme = ColorfulTheme::default();

//     let modes = &["🤖 Local Bot (offline)", "🌐 OpenAI API"];
//     let mode_index = Select::with_theme(&theme)
//         .with_prompt("Choose chat mode")
//         .items(modes)
//         .default(0)
//         .interact()
//         .unwrap();

//     let chat_mode = match mode_index {
//         0 => ChatMode::Local,
//         1 => ChatMode::OpenAI,
//         _ => unreachable!(),
//     };

//     let client = Client::new();
//     let mut messages: Vec<Message> = Vec::new();
//     let mut display_history: Vec<String> = Vec::new();

//     if matches!(chat_mode, ChatMode::OpenAI) {
//         messages.push(Message {
//             role: "system".to_string(),
//             content: "You are a helpful and friendly assistant. Keep responses concise."
//                 .to_string(),
//         });
//     }

//     println!("💬 ChatBot CLI - Type 'exit' to quit");
//     println!("💡 Use Up/Down arrows to recall previous messages\n");

//     let mut input_history = BasicHistory::new().max_entries(20).no_duplicates(true);

//     loop {
//         let input: String = Input::with_theme(&theme)
//             .with_prompt("You")
//             .history_with(&mut input_history)
//             .allow_empty(false)
//             .interact_text()
//             .unwrap();

//         if input.to_lowercase() == "exit" {
//             let confirmed = Confirm::with_theme(&theme)
//                 .with_prompt("Do you really want to quit?")
//                 .default(false)
//                 .interact()
//                 .unwrap();

//             if confirmed {
//                 println!("👋 Goodbye!");
//                 break;
//             }
//             continue;
//         }

//         display_history.push(format!("You: {}", input));

//         let response = match &chat_mode {
//             ChatMode::Local => bot_reply(&input),
//             ChatMode::OpenAI => {
//                 messages.push(Message {
//                     role: "user".to_string(),
//                     content: input.to_string(),
//                 });
//                 // // Now `?` works cleanly — reqwest::Error auto-converts to ChatError::Http
//                 // // No more `.map_err(|e| e.to_string())` boilerplate!
//                 ask_openai(&client, &mut messages).await?
//             }
//         };

//         let label = match &chat_mode {
//             ChatMode::Local => "Bot",
//             ChatMode::OpenAI => "AI",
//         };
//         println!("{}: {}", label, response);
//         display_history.push(format!("{}: {}", label, response));
//     }

//     println!("\n🗒️ Chat History:");
//     for line in &display_history {
//         println!("{}", line);
//     }

//     Ok(())
// }

// // // Return type changed: `Result<String, ChatError>` instead of `Result<String, String>`.
// // // The `?` operator now auto-converts errors via the #[from] impls we defined.
// async fn ask_openai(client: &Client, messages: &mut Vec<Message>) -> Result<String, ChatError> {
//     // // `env::var()?` now auto-converts VarError → ChatError::EnvVar via #[from]
//     // // No more `.expect()` panic — we get a proper error propagated to main
//     let api_key = env::var("OPENAI_API_KEY")?;

//     let request_body = ChatRequest {
//         model: "gpt-4o-mini".to_string(),
//         messages: messages.clone(),
//     };

//     // // `client.post(...)...send().await?` — the `?` converts reqwest::Error
//     // // to ChatError::Http automatically. Compare to Step 7 where we needed
//     // // `.map_err(|e| e.to_string())?` on every single call.
//     let response = client
//         .post("https://api.openai.com/v1/chat/completions")
//         .header("Content-Type", "application/json")
//         .header("Authorization", format!("Bearer {}", api_key))
//         .json(&request_body)
//         .send()
//         .await?;

//     // // Same here — `?` auto-converts the deserialization error
//     let body = response.json::<ChatResponse>().await?;

//     if let Some(choice) = body.choices.last() {
//         messages.push(Message {
//             role: "assistant".to_string(),
//             content: choice.message.content.clone(),
//         });
//         Ok(choice.message.content.clone())
//     } else {
//         // // This is the one error we construct manually — no library error to convert from
//         Err(ChatError::Api(
//             "No response choices returned by AI".to_string(),
//         ))
//     }
// }

// fn bot_reply(message: &str) -> String {
//     let msg = message.to_lowercase();

//     if msg.contains("hello") {
//         "Hi there!".to_string()
//     } else if msg.contains("how are you") {
//         "I'm just code, but I'm doing fine!".to_string()
//     } else if msg.contains("rust") {
//         "Rust is memory-safe and fearless!".to_string()
//     } else {
//         "I don't understand that yet.".to_string()
//     }
// }

// ⛳️ Step 8 - Beautiful error diagnostics with miette
// Run: cargo add miette --features fancy
//
// `miette` turns errors into rich diagnostic reports with:
// - Error codes like `chat::http_error` for searchability
// - `#[diagnostic(help("..."))]` messages suggesting what to do
// - `.wrap_err("context")` to add layers of context as errors propagate
// - Pretty-printed output with colors and arrows pointing to the cause
//
// `miette::Result<()>` replaces `Result<(), ChatError>` as the return type of main.
// It wraps any error that implements `Diagnostic` into a pretty report.
//
// `#[derive(Diagnostic)]` from miette works alongside `#[derive(Error)]` from thiserror.
// thiserror gives you the `Error` trait, miette adds diagnostic metadata on top.
//
// `.wrap_err("context message")` from `miette::Context` adds a contextual label
// around an error without losing the original cause — they stack up as a chain.
// This is like the garden tutorial's Step 4 where we added context to write errors.
use dialoguer::{BasicHistory, Confirm, Input, Select, theme::ColorfulTheme};
use dotenv::dotenv;
use miette::{Context, Diagnostic, IntoDiagnostic};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

// // `#[derive(Diagnostic)]` adds miette's diagnostic capabilities to our error.
// // `#[diagnostic(code(...))]` gives each error a searchable code like `chat::http_error`.
// // `#[diagnostic(help("..."))]` adds actionable suggestions shown in the error output.
// // These are displayed by miette's fancy reporter alongside the error message.
#[derive(Error, Diagnostic, Debug)]
enum ChatError {
    #[error("HTTP request failed")]
    #[diagnostic(
        code(chat::http_error),
        help("Check your internet connection and try again.")
    )]
    Http(#[from] reqwest::Error),

    // // This variant handles the specific case of a missing or invalid API key.
    // // The `help` message tells the user exactly what to do to fix it.
    #[error("missing API key: OPENAI_API_KEY not set")]
    #[diagnostic(
        code(chat::missing_api_key),
        help(
            "Create a .env file with: OPENAI_API_KEY=your_key_here\nGet a key at https://platform.openai.com/api-keys"
        )
    )]
    MissingApiKey(#[from] env::VarError),

    // // For API-level errors like "no response choices" or unexpected response formats.
    #[error("API returned an unexpected response: {0}")]
    #[diagnostic(
        code(chat::api_error),
        help("This may be a temporary issue. Try again in a moment.")
    )]
    Api(String),
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

enum ChatMode {
    Local,
    OpenAI,
}

// // `miette::Result<()>` wraps any `Diagnostic` error into a pretty report.
// // This replaces `Result<(), ChatError>` — miette handles the display.
#[tokio::main]
async fn main() -> miette::Result<()> {
    dotenv().ok();

    let theme = ColorfulTheme::default();

    let modes = &["🤖 Local Bot (offline)", "🌐 OpenAI API"];
    let mode_index = Select::with_theme(&theme)
        .with_prompt("Choose chat mode")
        .items(modes)
        .default(0)
        // // `.into_diagnostic()` converts any std Error into a miette Diagnostic.
        // // Useful for library errors that don't implement Diagnostic themselves.
        .interact()
        .into_diagnostic()?;

    let chat_mode = match mode_index {
        0 => ChatMode::Local,
        1 => ChatMode::OpenAI,
        _ => unreachable!(),
    };

    let client = Client::new();
    let mut messages: Vec<Message> = Vec::new();
    let mut display_history: Vec<String> = Vec::new();

    if matches!(chat_mode, ChatMode::OpenAI) {
        messages.push(Message {
            role: "system".to_string(),
            content: "You are a helpful and friendly assistant. Keep responses concise."
                .to_string(),
        });
    }

    println!("💬 ChatBot CLI - Type 'exit' to quit");
    println!("💡 Use Up/Down arrows to recall previous messages\n");

    let mut input_history = BasicHistory::new().max_entries(20).no_duplicates(true);

    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("You")
            .history_with(&mut input_history)
            .allow_empty(false)
            .interact_text()
            .into_diagnostic()?;

        if input.to_lowercase() == "exit" {
            let confirmed = Confirm::with_theme(&theme)
                .with_prompt("Do you really want to quit?")
                .default(false)
                .interact()
                .into_diagnostic()?;

            if confirmed {
                println!("👋 Goodbye!");
                break;
            }
            continue;
        }

        display_history.push(format!("You: {}", input));

        let response = match &chat_mode {
            ChatMode::Local => bot_reply(&input),
            ChatMode::OpenAI => {
                messages.push(Message {
                    role: "user".to_string(),
                    content: input.to_string(),
                });
                // // `.wrap_err("context")` adds a label around the error.
                // // If ask_openai fails, the user sees:
                // //   Error: failed to get AI response
                // //   Caused by: HTTP request failed
                // //   Help: Check your internet connection...
                // // The context stacks — each .wrap_err() adds a layer.
                ask_openai(&client, &mut messages)
                    .await
                    .wrap_err("failed to get AI response")?
            }
        };

        let label = match &chat_mode {
            ChatMode::Local => "Bot",
            ChatMode::OpenAI => "AI",
        };
        println!("{}: {}", label, response);
        display_history.push(format!("{}: {}", label, response));
    }

    println!("\n🗒️ Chat History:");
    for line in &display_history {
        println!("{}", line);
    }

    Ok(())
}

async fn ask_openai(client: &Client, messages: &mut Vec<Message>) -> Result<String, ChatError> {
    let api_key = env::var("OPENAI_API_KEY")?;

    let request_body = ChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages: messages.clone(),
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let body = response.json::<ChatResponse>().await?;

    if let Some(choice) = body.choices.last() {
        messages.push(Message {
            role: "assistant".to_string(),
            content: choice.message.content.clone(),
        });
        Ok(choice.message.content.clone())
    } else {
        Err(ChatError::Api(
            "No response choices returned by AI".to_string(),
        ))
    }
}

fn bot_reply(message: &str) -> String {
    let msg = message.to_lowercase();

    if msg.contains("hello") {
        "Hi there!".to_string()
    } else if msg.contains("how are you") {
        "I'm just code, but I'm doing fine!".to_string()
    } else if msg.contains("rust") {
        "Rust is memory-safe and fearless!".to_string()
    } else {
        "I don't understand that yet.".to_string()
    }
}
