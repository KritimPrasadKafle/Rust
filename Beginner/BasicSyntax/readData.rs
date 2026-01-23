// 6. Create a program that reads your name from stdin and greets you.
use std::io;

fn main(){
    let mut name = String::from("");
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // Remove any trailing newline or spaces
    println!("Hello, {}!", name);
}