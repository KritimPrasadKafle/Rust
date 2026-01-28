// 19. Write a program that converts seconds to hours, minutes, and seconds.

use std::io;

fn main() {
    let total_seconds = match read_number("Enter total seconds: ") {
        Some(n) => n,
        None => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;

    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    println!(
        "{} second(s) = {} hour(s), {} minute(s), {} second(s)",
        total_seconds, hours, minutes, seconds
    );
}

fn read_number(prompt: &str) -> Option<u64> {
    let mut input = String::new();
    println!("{}", prompt);

    io::stdin().read_line(&mut input).ok()?;
    let number: u64 = input.trim().parse().ok()?;

    Some(number)
}
