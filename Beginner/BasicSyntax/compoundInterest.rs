// 10. Create a program that calculates compound interest given principal, rate, and time.
use std::io;

fn main() {
    println!("Compound Interest Calculator");

    loop {
        let principal = match read_number("Enter principal amount (or 'q' to quit):") {
            Some(num) => num,
            None => {
                println!("Exiting calculator.");
                break;
            }
        };

        let rate = match read_number("Enter annual interest rate (in %):") {
            Some(num) => num,
            None => {
                println!("Exiting calculator.");
                break;
            }
        };

        let time = match read_number("Enter time in years:") {
            Some(num) => num,
            None => {
                println!("Exiting calculator.");
                break;
            }
        };

        let amount = calculate_compound_interest(principal, rate, time);
        println!("The amount after {:.2} years is: {:.2}\n", time, amount);
    }

    println!("Goodbye!");
}

// Read a floating point number from stdin. User can type 'q' to exit
fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;

    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("q") {
        return None;
    }

    trimmed.parse::<f64>().ok()
}

// Calculate compound interest
fn calculate_compound_interest(principal: f64, rate: f64, time: f64) -> f64 {
    principal * (1.0 + rate / 100.0).powf(time)
}
