// 17. Write a program that rounds a floating-point number to 2 decimal places.


fn main(){
    let num = match read_number("Enter a floating-point number:") {
        Some(num) => num,
        None => {
            println!("Invalid input for the number.");
            return;
        }
    };

    let rounded_num = round_to_two_decimal_places(num);
    println!("The number {:.4} rounded to 2 decimal places is {:.2}.", num, rounded_num);

}

fn read_number(prompt: &str) -> Option<f64> {
    use std::io;
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn round_to_two_decimal_places(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}