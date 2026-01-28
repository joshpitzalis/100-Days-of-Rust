use std::io::{self, Write};

fn main() {
    println!("🛠️ String Manipulation Tool");

    loop {
        println!("\nChoose an operation:");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
        println!("4. Trim");
        println!("5. Find Substring");
        println!("6. Replace Text");
        println!("7. Exit");

        let choice = prompt("Enter your choice: ");
        match choice.trim() {
            "1" => reverse(),
            "2" => uppercase(),
            "3" => lowercase(),
            "4" => trim(),
            "5" => find(),
            "6" => replace(),
            "7" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn reverse() {
    let s = prompt("Enter a string: ");
    let manipulation = s.chars().rev().collect::<String>();
    println!("🔄 Reversed: {}", manipulation);
}

fn uppercase() {
    let s = prompt("Enter a string: ");
    let manipulation = s.to_uppercase();
    println!("🔠 Uppercase: {}", manipulation);
}

fn lowercase() {
    let s = prompt("Enter a string: ");
    let manipulation = s.to_lowercase();
    println!("🔠 Lowercase: {}", manipulation);
}

fn trim() {
    let s = prompt("Enter a string: ");
    let manipulation = s.trim();
    println!("✂️ Trimmed: '{}'", manipulation);
}

fn find() {
    let s = prompt("Enter the main string: ");
    let sub = prompt("Enter substring to find: ");
    if s.contains(&sub) {
        println!("✅ Substring '{}' found!", sub);
    } else {
        println!("❌ Substring not found.");
    }
}

fn replace() {
    let s = prompt("Enter the main string: ");
    let old = prompt("Text to replace: ");
    let new = prompt("Replacement text: ");
    let manipulation = s.replace(&old, &new);
    println!("🔄 Result: {}", manipulation);
}
