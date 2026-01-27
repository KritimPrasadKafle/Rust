

// 16. Create a program that finds the absolute difference between two numbers.

fn main(){
    let num1 = match read_number("Enter the first number:") {
        Some(num) => num,
        None => {
            println!("Invalid input for the first number.");
            return;
        }
    };

    let num2 = match read_number("Enter the second number:") {
        Some(num) => num,
        None => {
            println!("Invalid input for the second number.");
            return;
        }
    };

    let abs_diff = absolute_difference(num1, num2);
    println!("The absolute difference between {:.2} and {:.2} is {:.2}.", num1, num2, abs_diff);
}


fn read_number(prompt: &str) -> Option<f64> {
    use std::io;
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn absolute_difference(a: f64, b: f64) -> f64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}