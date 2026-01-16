use ::std::env;
use ::std::fs::File;
use ::std::io::Read;

fn main() {
    // access the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("X Usage: Cargo run <file_path>");
        return;
    }
    let file_path = &args[1];
    println!("Reading file: {}...", file_path);

    // open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("X Error opening file: {}", err);
            return;
        }
    };

    // read the file contents
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }

    // count words
    let word_count = &contents.split_whitespace().count();
    println!("Word Count: {}", word_count);
}
