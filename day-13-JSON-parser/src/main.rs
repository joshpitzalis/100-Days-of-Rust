use serde_json::Value;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("❌ Usage: cargo run <path_to_json_file>");
        return;
    }

    let path = &args[1];
    match fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json) => println!(
                "✅ Parsed JSON:\n{}",
                serde_json::to_string_pretty(&json).unwrap()
            ),
            Err(e) => eprintln!("❌ Invalid JSON: {}", e),
        },
        Err(e) => eprintln!("❌ Failed to read file: {}", e),
    }
}

// Ok... how do you pronounce serde?
// https://www.reddit.com/r/rust/comments/p3qsbc/ok_how_do_you_pronounce_serde/
