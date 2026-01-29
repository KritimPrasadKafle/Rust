// 49. Write a program that checks if a number is prime.

fn main(){
    let number = match read_number("Enter a number: ") {
        Some(num) => num,
        None => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };
    let result = is_prime(number);
    if result {
        println!("{} is a prime number.", number);

    } else {
        println!("{} is not a prime number.", number);
    }
}

fn read_number(prompt: &str) -> Option<i32> {
    use std::io;
    let mut input = String::new();
    println!("{}", prompt);
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<i32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        },
        Err(_) => None,
    }
}


fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as i32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}