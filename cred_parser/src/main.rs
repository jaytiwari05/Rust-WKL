use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    // Read the path argument. Default to passwords.txt if none given.
    let args: Vec<String> = env::args().collect();
    let path: &str = if args.len() >= 2 {
        &args[1]
    } else {
        "passwords.txt"
    };

    // Open the file. Bail out cleanly on error.
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open '{path}': {e}");
            process::exit(1);
        }
    };

    // Wrap in a buffered reader for line-by-line streaming. Declared
    // `mut` because `read_line` takes `&mut self`.
    let mut reader = BufReader::new(file);

    // Counters live across iterations. They mutate, so they are `mut`.
    let mut total: u64 = 0;
    let mut accepted: u64 = 0;
    let mut rejected: u64 = 0;

    // Reuse one allocation for the line buffer across every iteration.
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break, // 0 bytes = end of file
            Ok(_) => {}
            Err(e) => {
                eprintln!("Read error: {e}");
                process::exit(2);
            }
        }
        total += 1;

        // Take a string slice into the buffer. No allocation.
        let trimmed: &str = line.trim_end();

        if trimmed.is_empty() {
            continue;
        }

        if classify(trimmed) {
            accepted += 1;
            println!("[OK]   {trimmed}");
        } else {
            rejected += 1;
            println!("[SKIP] {trimmed}");
        }
    }

    println!();
    println!("=== Summary ===");
    println!("Total:    {total}");
    println!("Accepted: {accepted}");
    println!("Rejected: {rejected}");
}

// Borrows the candidate as &str. Returns true if it passes the policy.
fn classify(candidate: &str) -> bool {
    candidate.len() >= 8
}
