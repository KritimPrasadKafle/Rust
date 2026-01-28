// 18. Create a program that calculates simple interest.

use std::io;
fn main(){

    let principal = match read_number("Please enter principal."){
        Some(n) => n,
        None => {
            println!("Provide number instead of other things.");
            return;
        }
    };

    let time = match read_number("Please enter time."){
        Some(n) => n,
        None => {
            println!("Provide number instead of other things.");
            return;
        }
    };

    let rate = match read_number("Please enter rate."){
        Some(n) => n,
        None => {
            println!("Provide number instead of other things.");
            return;
        }
    };

    let simple_interest = simple_interest(principal, rate, time);
    println!("The simple interest is: {}", simple_interest);




}

fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    let number: f64 = input.trim().parse().ok()?;
    Some(number)
}


fn simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    (principal * rate * time) / 100.0
}
