
// 15. Write a program that calculates the percentage of a number.

fn main(){
    let total = match read_number("Enter the total number:") {
        Some(num) => num,
        None => {
            println!("Invalid input for total number.");
            return;
        }
    };

    let percentage = match read_number("Enter the percentage to calculate:") {
        Some(num) => num,
        None => {
            println!("Invalid input for percentage.");
            return;
        }
    };

    let result = calculate_percentage(total, percentage);
    println!("{:.2}% of {:.2} is {:.2}.", percentage, total, result);
}

fn read_number(prompt: &str) -> Option<f64> {
    use std::io;
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn calculate_percentage(total: f64, percentage: f64) -> f64 {
    (percentage / 100.0) * total
}
