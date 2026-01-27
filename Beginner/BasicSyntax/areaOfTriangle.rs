// 13. Write a program that calculates the area of a triangle given base and height.

fn main(){

    let base = match read_number("Enter the base of the triangle:") {
        Some(num) => num,
        None => {
            println!("Invalid input for base.");
            return;
        }
    };

    let height = match read_number("Enter the height of the triangle:") {
        Some(num) => num,
        None => {
            println!("Invalid input for height.");
            return;
        }
    };

    let area = calculate_area(base, height);
    println!("The area of the triangle is {:.2}.", area);
    





}

fn read_number(prompt: &str) -> Option<f64> {
    use std::io;
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn calculate_area(base: f64, height: f64) -> f64 {
    0.5 * base * height
}