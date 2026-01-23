use rand::Rng;
use std;

fn main() {
    println!("Rock, Paper, Scissors!");
    loop {
        println!("Let's play...make your choice: Rock(r), Paper,(p) or Scissors(s)");
        let user_choice = get_user_choice();
        let computer_choice = get_computer_choice();
        println!("Computer be deciding...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Computer chose {}", computer_choice.to_uppercase());
        match decide_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You Win!"),
            GameResult::Lose => println!("You lose!"),
            GameResult::Draw => println!("It's a draw."),
        }
    }
}

fn get_user_choice() -> String {
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");
    let choice = choice.trim().to_lowercase();
    match choice.as_str() {
        "rock" | "r" => "rock".to_string(),
        "paper" | "p" => "paper".to_string(),
        "scissors" | "s" => "scissors".to_string(),
        _ => {
            println!("Invalid choice. Try again.");
            get_user_choice()
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let index = rand::rng().random_range(0..choices.len());
    choices[index].to_string()
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn decide_winner(user: &str, computer: &str) -> GameResult {
    match (user, computer) {
        ("rock", "scissors") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        (a, b) if a == b => GameResult::Draw,
        _ => GameResult::Lose,
    }
}
