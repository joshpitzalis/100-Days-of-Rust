fn main() {
    println!("Enter time duration (format: hours minutes seconds)");
    let duration = match parse_duration(){
        Some(dur) => dur,
        None => {
            println!("Invalid format. Please eneter numbers only (e.g., 0 1 30 for 1 minute 30 seconds.")
        }
    }
    println!("Timer set for {} hours, {} minutes, {} seconds", duration.0, duratiion.1, duration.2);
    start_timer(duration);
    print!("Times's Up!")
}
