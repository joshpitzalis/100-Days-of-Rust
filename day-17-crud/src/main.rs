use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    id: usize,
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n📇 Contact Manager:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search Contact");
        println!("4. Delete Contact");
        println!("5. Exit");

        let choice = input("Enter your choice: ");
        match choice.trim() {
            "1" => add_contact(&mut contacts, &mut next_id),
            "2" => view_contact(&contacts),
            "3" => search_contact(&contacts),
            "4" => delete_contact(&mut contacts),
            "5" => {
                println!("👋 Exiting...");
                break;
            }
            _ => println!("❌ Invalid option."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_contact(contacts: &mut Vec<Contact>, next_id: &mut usize) {
    let name = input("Enter name: ");
    let phone = input("Enter phone: ");
    let email = input("Enter email: ");

    contacts.push(Contact {
        id: next_id.clone(),
        name,
        phone,
        email,
    });
    println!("✅ Contact added.");
    *next_id += 1;
}

fn view_contact(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        println!("📭 No contacts.");
    } else {
        for c in contacts {
            println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
        }
    }
}

fn search_contact(contacts: &Vec<Contact>) {
    let query = input("Search by name or email: ");
    let results: Vec<&Contact> = contacts
        .iter()
        .filter(|c| c.name.contains(&query) || c.email.contains(&query))
        .collect();

    if results.is_empty() {
        println!("❌ No match found.");
    } else {
        for c in results {
            println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
        }
    }
}

fn delete_contact(contacts: &mut Vec<Contact>) {
    let id = input("Enter ID to delete: ").parse::<usize>().unwrap_or(0);
    if let Some(index) = contacts.iter().position(|c| c.id == id) {
        contacts.remove(index);
        println!("🗑️ Contact deleted.");
    } else {
        println!("❌ ID not found.");
    }
}
