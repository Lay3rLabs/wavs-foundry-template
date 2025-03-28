use std::io::{self, Read};

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let idx = match input.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid input.  Please provide a non-negative integer.");
            return;
        }
    };

    let result = fib(idx);
    println!("Fibonacci sequence number at index {} is {}", idx, result);
}
