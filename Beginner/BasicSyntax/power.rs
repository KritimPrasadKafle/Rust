// 20. Create a program that calculates the power of a number (x^y).

fn main(){
    let number = match read_number("Enter the base number (x): "){
        Some(n) => n,
        None => {
            println!("Invalid input for base number.");
            return;
        }
    };

    let exponent = match read_number("Enter the exponent (y): "){
        Some(n) => n,
        None => {
            println!("Invalid input for exponent.");
            return;
        }
    };

    let result = power(number, exponent);
    println!("{} raised to the power of {} is {}", number, exponent, result);


}

fn read_number(prompt: &str) -> Option<f64> {
    use std::io;
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    let number: f64 = input.trim().parse().ok()?;
    Some(number)
}

fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}