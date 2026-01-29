// 45. Write a program that prints the multiplication table of a number.

use::std::io;

fn main(){
    let number = match read_numbers("Enter number"){
        Some(n) => n,
        None => {
            println!("Invalid input");
            return;
        }
    };
    for (index, value) in (1..=10).enumerate(){
        println!("{} x {} = {}", number, index + 1, number * value);
    }
}


fn read_numbers(prompt: &str) -> Option<i32>{
    let mut input = String::new();
    println!("{}", prompt);
    match io::stdin().read_line(&mut input){
        Ok(_) => match input.trim().parse::<i32>(){
            Ok(num) => Some(num),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

