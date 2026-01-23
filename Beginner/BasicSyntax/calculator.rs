// 7. Write a calculator that adds, subtracts, multiplies, and divides two numbers.
// use std::io;
// fn main(){
//     let mut num1 = String::new();
//     let mut num2 = String::new();
//     let mut operation = String::new();
//     println!("Enter first number: ");
//     io::stdin().read_line(&mut num1)
// .expect("Failed to read line");
//     let num1 = parse_and_trim(num1);
//     println!("Enter second number: ");
//     io::stdin().read_line(&mut num2)
// .expect("Failed to read line");
//     let num2 = parse_and_trim(num2);
//     println!("Enter operation (+, -, *, /): ");
//     io::stdin().read_line(&mut operation)
// .expect("Failed to read line");
//     let operation = operation.trim();
//     let result = match operation {
//         "+" => add(num1, num2),
//         "-" => subtracts(num1, num2),
//         "*" => multiplies(num1, num2),
//         "/" => divides(num1, num2),
//         _ => {
//             println!("Invalid operation");
//             return;
//         }
//     };
//     println!("Result: {}", result);
// }

// fn add(a:f64, b:f64) -> f64{
//     a+b
// }

// fn subtracts(a:f64, b:f64) -> f64{
//     a-b
// }

// fn multiplies(a:f64, b:f64) -> f64{
//     a*b
// }
// fn divides(a:f64, b:f64) -> f64{
//     a/b
// }

// fn parse_and_trim(s: String) -> f64{
//     s.trim().parse::<f64>().expect("Please type a number!")
// }



use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    println!("🧮 Rust Calculator (type 'q' to quit)");

    loop {
        let num1 = match read_number("Enter first number: ") {
            Some(n) => n,
            None => break,
        };

        let num2 = match read_number("Enter second number: ") {
            Some(n) => n,
            None => break,
        };

        let operation = match read_operation() {
            Some(op) => op,
            None => break,
        };

        match calculate(num1, num2, operation) {
            Ok(result) => println!("✅ Result: {}\n", result),
            Err(err) => println!("❌ Error: {}\n", err),
        }
    }

    println!("👋 Goodbye!");
}


fn read_number(prompt: &str) -> Option<f64> {
    let mut input = String::new();
    println!("{}", prompt);

    io::stdin().read_line(&mut input).ok()?;

    let input = input.trim();
    if input.eq_ignore_ascii_case("q") {
        return None;
    }

    input.parse::<f64>().ok()
}

fn read_operation() -> Option<Operation> {
    let mut input = String::new();
    println!("Enter operation (+, -, *, /):");

    io::stdin().read_line(&mut input).ok()?;

    match input.trim() {
        "+" => Some(Operation::Add),
        "-" => Some(Operation::Subtract),
        "*" => Some(Operation::Multiply),
        "/" => Some(Operation::Divide),
        "q" | "Q" => None,
        _ => {
            println!("❌ Invalid operation");
            Some(read_operation()?)
        }
    }
}

fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}
