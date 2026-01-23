// 11. Write a program that finds if a number is even or odd.
use std::io;

fn main() {
    let number = match read_number("Enter the number:") {
        Some(num) => num,
        None => {
            println!("Invalid input.");
            return;
        }
    };

    if even(number) {
        println!("The number {} is even.", number);
    } else {
        println!("The number {} is odd.", number);
    }
}

fn read_number(prompt: &str) -> Option<i32> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<i32>().ok()
}

fn even(num: i32) -> bool {
    num % 2 == 0
}
