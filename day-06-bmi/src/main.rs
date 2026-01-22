use std::io;

fn main() {
    println!("BMI Calculator");
    println!("Please enter your weight in kilograms:");

    let weight = match get_input_as_number() {
        Some(weight) => weight,
        None => {
            println!("Invalid input for weight. Enter a valid number.");
            return;
        }
    };

    println!("Please enter your height in meters:");

    let height = match get_input_as_number() {
        Some(height) => height,
        None => {
            println!("Invalid input for height. Enter a valid number.");
            return;
        }
    };

    if height == 0.0 {
        println!("Height cannot be zero.");
        return;
    }

    let bmi = get_bmi(weight, height);

    println!("{:.2} BMI", bmi);

    let category = calculate_bmi(bmi);
    println!("You are {}", category);
}

fn get_input_as_number() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("X Failed to read line");

    match input.trim().parse::<f64>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

fn get_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn calculate_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "underweight"
    } else if bmi < 25.0 {
        "normal weight"
    } else if bmi < 30.0 {
        "overweight"
    } else {
        "obese"
    }
}
