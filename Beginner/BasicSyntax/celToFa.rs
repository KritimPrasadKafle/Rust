// 8. Create a program that converts Celsius to Fahrenheit and Kelvin.

// fn main(){
//     let celsius = 25.0;
//     let fahrenheit = celsius_to_fahrenheit(celsius);
//     let kelvin = celsius_to_kelvin(celsius);
//     println!("Celsius: {}°C", celsius);
//     println!("Fahrenheit: {}°F", fahrenheit);
//     println!("Kelvin: {}K", kelvin);

// }

// fn celsius_to_fahrenheit(c: f64) -> f64{
//     (c * 9.0/5.0) + 32.0
// }
// fn celsius_to_kelvin(c: f64) -> f64{
//     c + 273.15
// }


use std::io;

#[derive(Copy, Clone)]

enum Conversion {
    CelsiusToFahrenheit,
    CelsiusToKelvin,
}

fn main() {
    println!("Select conversion type:");

    let celsius = match read_number("Enter temperature in Celsius: ") {
        Some(num) => num,
        None => {
            println!("Invalid input for Celsius temperature.");
            return;
        }
    };

    let conversion = match read_conversion() {
        Some(conv) => conv,
        None => {
            println!("Invalid conversion type selected.");
            return;
        }
    };

    let result = convert(celsius, conversion);

    match conversion {
        Conversion::CelsiusToFahrenheit => println!("{}°C is {}°F", celsius, result),
        Conversion::CelsiusToKelvin => println!("{}°C is {}K", celsius, result),
    }
}

fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn read_conversion() -> Option<Conversion> {
    let mut input = String::new();
    println!("1. Celsius to Fahrenheit");
    println!("2. Celsius to Kelvin");
    println!("Enter choice (1 or 2): ");
    io::stdin().read_line(&mut input).ok()?;

    match input.trim() {
        "1" => Some(Conversion::CelsiusToFahrenheit),
        "2" => Some(Conversion::CelsiusToKelvin),
        _ => None,
    }
}

fn convert(celsius: f64, conversion: Conversion) -> f64 {
    match conversion {
        Conversion::CelsiusToFahrenheit => (celsius * 9.0 / 5.0) + 32.0,
        Conversion::CelsiusToKelvin => celsius + 273.15,
    }
}
