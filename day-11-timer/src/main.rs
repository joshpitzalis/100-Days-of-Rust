use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Enter time duration (format: hours minutes seconds)");
    let duration = match get_timer_input() {
        Some(dur) => dur,
        None => {
            println!(
                "Invalid format. Please eneter numbers only (e.g., 0 1 30 for 1 minute 30 seconds."
            );
            return;
        }
    };
    println!(
        "Timer set for {} hours, {} minutes, {} seconds",
        duration.0, duration.1, duration.2
    );
    start_timer(duration);
    print!("Times's Up!")
}

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;
    Some((hours, minutes, seconds))
}

fn start_timer(duration: (u64, u64, u64)) {
    let total_seconds = duration.0 * 3600 + duration.1 * 60 + duration.2;
    for i in (1..=total_seconds).rev() {
        let hrs = i / 3600;
        let mins = (i % 3600) / 60;
        let secs = i % 60;
        print!("Time remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!()
}

// Why is flushing needed?

// Standard output is typically **line-buffered**, meaning:
// - Output is automatically flushed when a newline (`\n`) is encountered
// - Output may be held in a buffer until the buffer is full or the program ends

// This becomes important when you want to display output immediately, especially for:
// - Progress indicators
// - Prompts that don't end with newlines
// - Real-time status updates

// Without the `flush()` call, "Time remaining: " might not appear every second, making the program seem unresponsive.
//
// In this scenario you likely won't notice much difference because the **`println!` macro already includes a newline character** (`\n`), which automatically triggers a flush.
//
// However, there are some scenarios where removing `flush()` could cause issues:
//
// 1. **Terminal buffering policies**: Some terminals or environments might use full buffering instead of line buffering, especially when output is redirected to a file
//
// 2. **System under load**: Heavy system load might delay automatic flushing
//
// 3. **Different platforms**: Buffering behavior can vary between operating systems
//
// You can replace  println! with  print! and uncomment the flush line to break things. Without the flush you won't see the count down at all until the timer completes.
