fn main() {
    println!("Enter a number to check if its a prime number:");
    let number = get_valid_number();
    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is NOT a prime number.", number);
    }
}

fn get_valid_number() -> u32 {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}
