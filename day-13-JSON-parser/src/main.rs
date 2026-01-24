// cargo add serde_json
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

// // HTTP APIs, WebSockets, or any network communication:
// // rust// Sending API response
// let response = ApiResponse { status: "success", data: user };
// let json_string = serde_json::to_string(&response)?;
// // Send this string via HTTP, WebSocket, etc.
// Just like in TypeScript when you do fetch(url, { body: JSON.stringify(data) })

// // Logging/Debugging
// Pretty-printing complex structs for inspection:

// rustprintln!("{}", serde_json::to_string_pretty(&complex_data)?);

// // Storing in Databases
// Many databases (Postgres JSONB, MongoDB, etc.) accept JSON strings:

// let metadata = UserMetadata { preferences: ... };
// let json = serde_json::to_string(&metadata)?;
// db.execute("INSERT INTO users (metadata) VALUES (?)", &json)?;
