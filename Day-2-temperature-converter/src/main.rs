use std::io;

fn main() {
    println!("Welcome to the temperature converter 🌡️");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Please enter your choice (1 or 2):");

    let choice: u8 = {
        let mut attempts = 0;

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse() {
                Ok(num) => {
                    if num == 1 || num == 2 {
                        break num;
                    } else {
                        attempts += 1;
                        if attempts >= 3 {
                            println!("Too many invalid attempts. Exiting.");
                            return;
                        }
                        println!("Invalid choice. Please enter 1 or 2:");
                    }
                }
                Err(_) => {
                    attempts += 1;
                    if attempts >= 3 {
                        println!("Too many invalid attempts. Exiting.");
                        return;
                    }
                    println!("Please type a number! Try again:");
                }
            }
        }
    };

    match choice {
        1 => celsius_to_fahrenheit(),
        2 => fahrenheit_to_celsius(),
        _ => println!("Invalid choice"),
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let celsius: f64 = input.trim().parse().expect("Please type a number!");

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C is {:.2}°F", celsius, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let fahrenheit: f64 = input.trim().parse().expect("Please type a number!");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}°F is {:.2}°C", fahrenheit, celsius);
}
