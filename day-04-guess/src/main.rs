use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game 💭");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    let secret_number = rand::rng().random_range(0..=100);
    let mut attempts = 0;

    loop {
        println!("Place your bets:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("X Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("A number between 1 and 100. Try again.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("X Please enter a valid number");
                continue;
            }
        };
        println!("You guessed {}", guess);

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Low. Try again.");
            }
            Ordering::Greater => {
                println!("Too High. Try again.");
            }
            Ordering::Equal => {
                println!("Congratulations! You guessed it in {} attempts", attempts);
                break;
            }
        }
    }
}
