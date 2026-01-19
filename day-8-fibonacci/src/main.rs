use std::io;

fn main() {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of terms you want to generate");

    let num_terms = match get_input_as_u32() {
        Some(value) => {
            if value == 0 {
                println!("X Number must be greater than 0");
                return;
            }
            value
        }
        None => {
            println!("X Invalid input.");
            return;
        }
    };

    let sequence = generate_fibonacci(num_terms);
    println!("Fibonacci Sequence ({} terms): {:?}", num_terms, sequence)
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    match input.trim().parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n >= 1 {
        sequence.push(0);
    }

    if n >= 2 {
        sequence.push(1);
    }

    for i in 2..n {
        let a: u64 = sequence[i as usize - 1];
        let b: u64 = sequence[i as usize - 2];

        match a.checked_add(b) {
            Some(next) => sequence.push(next),
            None => {
                println!("⚠ Overflow detected at term {}. Stopping sequence.", i + 1);
                break;
            }
        }
    }

    sequence
}
