// 9. Write a program that calculates BMI (Body Mass Index) from height and weight.
use std::io;
fn main() {
    let weight = match read_number("Enter weight in kilograms:"){
        Some(num) => num,
        None => {
            println!("Invalid input for weight.");
            return;
        }
    };
    let height = match read_number("Enter height in meters:"){
        Some(num) => num,
        None => {
            println!("Invalid input for height.");
            return;
        }
    };

    match bmi(weight, height) {
        Some(bmi_value) => println!("Your BMI is {:.2}", bmi_value),
        None => println!("Height must be greater than zero to calculate BMI."),
    }

}


fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn bmi(weight: f64, height: f64) -> Option<f64> {
    if height <= 0.0 {
        None
    } else {
        Some(weight / (height * height))
    }
}