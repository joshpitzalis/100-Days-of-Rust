use std::io::{self, Write};

#[derive(Debug, Clone)]
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
            "1" => (contacts, next_id) = add_contact(contacts, next_id),
            "2" => contacts = view_contact(contacts),
            "3" => contacts = search_contact(contacts),
            "4" => contacts = delete_contact(contacts),
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

// fn add_contact(contacts: &mut Vec<Contact>, next_id: &mut usize) {
//     let name = input("Enter name: ");
//     let phone = input("Enter phone: ");
//     let email = input("Enter email: ");

//     contacts.push(Contact {
//         id: next_id.clone(),
//         name,
//         phone,
//         email,
//     });
//     println!("✅ Contact added.");
//     *next_id += 1;
// }

fn add_contact(mut contacts: Vec<Contact>, next_id: usize) -> (Vec<Contact>, usize) {
    let name = input("Enter name: ");
    let phone = input("Enter phone: ");
    let email = input("Enter email: ");

    contacts.push(Contact {
        id: next_id,
        name,
        phone,
        email,
    });
    println!("✅ Contact added.");
    (contacts, next_id + 1)
}

fn view_contact(contacts: Vec<Contact>) -> Vec<Contact> {
    if contacts.is_empty() {
        println!("📭 No contacts.");
    } else {
        for c in contacts.clone() {
            // contacts is now GONE - moved into the iterator
            println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
        }
    };
    contacts
}

fn search_contact(contacts: Vec<Contact>) -> Vec<Contact> {
    let query = input("Search by name or email: ");
    let results: Vec<Contact> = contacts
        .iter()
        .filter(|c| c.name.contains(&query) || c.email.contains(&query))
        .cloned()
        .collect();

    if results.is_empty() {
        println!("❌ No match found.");
    } else {
        for c in results {
            println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
        }
    }
    contacts
}

// fn delete_contact(contacts: &mut Vec<Contact>) {
//     let id = input("Enter ID to delete: ").parse::<usize>().unwrap_or(0);
//     if let Some(index) = contacts.iter().position(|c| c.id == id) {
//         contacts.remove(index);
//         println!("🗑️ Contact deleted.");
//     } else {
//         println!("❌ ID not found.");
//     }
// }

fn delete_contact(mut contacts: Vec<Contact>) -> Vec<Contact> {
    let id = input("Enter ID to delete: ").parse::<usize>().unwrap_or(0);
    if let Some(index) = contacts.iter().position(|c| c.id == id) {
        contacts.remove(index);
        println!("🗑️ Contact deleted.");
    } else {
        println!("❌ ID not found.");
    }
    contacts
}

// use std::io::{self, Write};

// #[derive(Debug)]
// struct Contact {
//     id: usize,
//     name: String,
//     phone: String,
//     email: String,
// }

// fn main() {
//     let mut contacts: Vec<Contact> = Vec::new();
//     let mut next_id = 1;

//     loop {
//         println!("\n📇 Contact Manager:");
//         println!("1. Add Contact");
//         println!("2. View Contacts");
//         println!("3. Search Contact");
//         println!("4. Delete Contact");
//         println!("5. Exit");

//         let choice = input("Enter your choice: ");
//         match choice.trim() {
//             "1" => {
//                 let name = input("Name: ");
//                 let phone = input("Phone: ");
//                 let email = input("Email: ");
//                 contacts.push(Contact { id: next_id, name, phone, email });
//                 println!("✅ Contact added with ID {}", next_id);
//                 next_id += 1;
//             }
//             "2" => {
//                 if contacts.is_empty() {
//                     println!("📭 No contacts.");
//                 } else {
//                     for c in &contacts {
//                         println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
//                     }
//                 }
//             }
//             "3" => {
//                 let query = input("Search by name or email: ");
//                 let results: Vec<&Contact> = contacts.iter()
//                     .filter(|c| c.name.contains(&query) || c.email.contains(&query))
//                     .collect();
//                 if results.is_empty() {
//                     println!("❌ No match found.");
//                 } else {
//                     for c in results {
//                         println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
//                     }
//                 }
//             }
//             "4" => {
//                 let id = input("Enter ID to delete: ").parse::<usize>().unwrap_or(0);
//                 let len_before = contacts.len();
//                 contacts.retain(|c| c.id != id);
//                 if contacts.len() < len_before {
//                     println!("🗑️ Contact deleted.");
//                 } else {
//                     println!("❌ ID not found.");
//                 }
//             }
//             "5" => {
//                 println!("👋 Exiting...");
//                 break;
//             }
//             _ => println!("❌ Invalid option."),
//         }
//     }
// }

// fn input(prompt: &str) -> String {
//     print!("{}", prompt);
//     io::stdout().flush().unwrap();
//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();
//     buf.trim().to_string()
// }
