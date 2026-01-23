// 12. Create a program that converts kilometers to miles and vice versa.
use std::io;

enum Conversion {
    KmToMiles,
    MilesToKm,
}

fn main() {
    println!("Distance Converter");
    println!("1. Kilometers to Miles");
    println!("2. Miles to Kilometers");

    let choice = match read_number("Enter your choice (1 or 2):") {
        Some(num) => num as i32,
        None => {
            println!("Invalid choice.");
            return;
        }
    };

    let conversion = match choice {
        1 => Conversion::KmToMiles,
        2 => Conversion::MilesToKm,
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    let distance = match read_number("Enter the distance:") {
        Some(num) => num,
        None => {
            println!("Invalid input for distance.");
            return;
        }
    };

    let result = convert(distance, conversion);

    match conversion {
        Conversion::KmToMiles => println!("{:.2} km is {:.2} miles.", distance, result),
        Conversion::MilesToKm => println!("{:.2} miles is {:.2} km.", distance, result),
    }
}

// Reads a floating point number from stdin
fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

// Performs the conversion
fn convert(distance: f64, conversion: Conversion) -> f64 {
    match conversion {
        Conversion::KmToMiles => distance * 0.621371,
        Conversion::MilesToKm => distance / 0.621371,
    }
}
