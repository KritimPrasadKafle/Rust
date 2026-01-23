fn main(){
    let age = String::from("42");
    let age = age.parse::<i32>().unwrap();
    println!("I am {} years old.", age);
}

// unwrap() is a method in Rust that extracts the value from a Result or Option type.

// .unwrap() says: "Give me the value inside, or crash the program if there's an error"
