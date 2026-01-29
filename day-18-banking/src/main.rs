use std::io::{self, Write};

#[derive(Debug)]
struct Account {
    id: usize,
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n🏦 Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        match input("Choose an option: ").as_str() {
            "1" => {
                let result = create_account(accounts, next_id);
                accounts = result.0;
                next_id = result.1;
            }
            "2" => accounts = view_balance(accounts),
            "3" => accounts = deposit(accounts),
            "4" => accounts = withdraw(accounts),
            "5" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }

        // if let "1" = choice.as_str() {
        //     let result = create_account(accounts, next_id);
        //     accounts = result.0;
        //     next_id = result.1;
        // } else if let "2" = choice.as_str() {
        //     accounts = view_balance(accounts);
        // } else if let "3" = choice.as_str() {
        //     accounts = deposit(accounts);
        // } else if let "4" = choice.as_str() {
        //     accounts = withdraw(accounts);
        // } else if let "5" = choice.as_str() {
        //     println!("👋 Goodbye!");
        //     break;
        // } else {
        //     println!("❌ Invalid choice.");
        // }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn create_account(mut accounts: Vec<Account>, next_id: usize) -> (Vec<Account>, usize) {
    let name = input("Account holder name: ");
    let amount = input("Initial deposit: ").parse::<f64>().unwrap_or(0.0);
    accounts.push(Account {
        id: next_id,
        name,
        balance: amount,
    });
    println!("✅ Account created with ID: {}", next_id);
    (accounts, next_id + 1)
}

// fn view_balance(accounts: Vec<Account>) -> Vec<Account> {
//     let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
//     match accounts.iter().find(|acc| acc.id == id) {
//         Some(acc) => println!("💼 {} | Balance: {:.2}", acc.name, acc.balance),
//         None => println!("❌ Account not found."),
//     }
//     accounts
// }

fn view_balance(accounts: Vec<Account>) -> Vec<Account> {
    let id = input("Account ID: ").parse::<usize>().unwrap_or(0);

    if let Some(acc) = accounts.iter().find(|acc| acc.id == id) {
        println!("💼 {} | Balance: {:.2}", acc.name, acc.balance);
    } else {
        println!("❌ Account not found.");
    }
    accounts
}

// fn deposit(mut accounts: Vec<Account>) -> Vec<Account> {
//     let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
//     let amt = input("Amount to deposit: ").parse::<f64>().unwrap_or(0.0);
//     match accounts.iter_mut().find(|acc| acc.id == id) {
//         Some(acc) => {
//             acc.balance += amt;
//             println!("💰 New balance: {:.2}", acc.balance);
//         }
//         None => println!("❌ Account not found."),
//     }
//     accounts
// }

fn deposit(mut accounts: Vec<Account>) -> Vec<Account> {
    let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
    let amt = input("Amount to deposit: ").parse::<f64>().unwrap_or(0.0);

    if let Some(acc) = accounts.iter_mut().find(|acc| acc.id == id) {
        acc.balance += amt;
        println!("💰 New balance: {:.2}", acc.balance);
    } else {
        println!("❌ Account not found.")
    }
    accounts
}

// fn withdraw(mut accounts: Vec<Account>) -> Vec<Account> {
//     let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
//     let amt = input("Amount to withdraw: ").parse::<f64>().unwrap_or(0.0);
//     match accounts.iter_mut().find(|acc| acc.id == id) {
//         Some(acc) => {
//             if acc.balance >= amt {
//                 acc.balance -= amt;
//                 println!("🏧 Withdrawal successful. New balance: {:.2}", acc.balance);
//             } else {
//                 println!("❌ Insufficient funds.");
//             }
//         }
//         None => println!("❌ Account not found."),
//     };
//     accounts
// }

fn withdraw(mut accounts: Vec<Account>) -> Vec<Account> {
    let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
    let amt = input("Amount to withdraw: ").parse::<f64>().unwrap_or(0.0);

    if let Some(acc) = accounts.iter_mut().find(|acc| acc.id == id) {
        if acc.balance >= amt {
            acc.balance -= amt;
            println!("🏧 Withdrawal successful. New balance: {:.2}", acc.balance);
        } else {
            println!("❌ Insufficient funds.");
        }
    } else {
        println!("❌ Account not found.")
    }
    accounts
}
