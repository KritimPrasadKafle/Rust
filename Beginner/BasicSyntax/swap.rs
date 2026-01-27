// 14. Create a program that swaps two numbers without using a temporary variable.

use std::io;

fn main(){
    let mut num1 = match read_number("Enter the first number:") {
        Some(num) => num,
        None => {
            println!("Invalid input for the first number.");
            return;
        }
    };

    let mut num2 = match read_number("Enter the second number:") {
        Some(num) => num,
        None => {
            println!("Invalid input for the second number.");
            return;
        }
    };

    println!("Before swapping: num1 = {:.2}, num2 = {:.2}", num1, num2);

    // Swapping without a temporary variable

    swap_numbers(&mut num1, &mut num2);
  

    println!("After swapping: num1 = {:.2}, num2 = {:.2}", num1, num2);
}


fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn swap_numbers(a: &mut f64, b: &mut f64) {
    *a = *a + *b;
    *b = *a - *b;
    *a = *a - *b;
}