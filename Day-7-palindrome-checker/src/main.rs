use std::{io, string};

fn main() {
    println!("Palindrone Checker");
    println!("Enter a string to check if its a palindrome.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let clean_input = clean_string(&input);
    if clean_input.is_empty() {
        println!("Please enter a valid string with actual letters in it.");
        return;
    }

    if is_palindrome(&clean_input) {
        println!("{} is a palindrome.", clean_input.trim())
    } else {
        println!("{} is NOT a palindrome.", clean_input.trim())
    }
}

fn clean_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>()
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}
