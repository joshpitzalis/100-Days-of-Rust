fn main() {
    println!("Simple Calculator");
    println!("Enter your expression:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let expression: Vec<&str> = input.trim().split_whitespace().collect();
    if expression.len() != 3 {
        println!(
            "Invalid expression. Please follow the format 'number operator number' with spaces between each element."
        );
        return;
    }

    let num1: f64 = match expression[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid first number");
            return;
        }
    };

    let num2: f64 = match expression[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid second number");
            return;
        }
    };

    let operator = expression[1];
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => divide(num1, num2),
        _ => panic!("Invalid operator"),
    };

    println!("Result: {}", result);
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        panic!("Division by zero not allowed.");
    }
    num1 / num2
}
