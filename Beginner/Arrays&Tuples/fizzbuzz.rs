// 43. Write a program that implements FizzBuzz for numbers 1-100.

fn main(){
    for i in 1..=100{
        if i % 3 == 0 && i % 5 == 0{
            println!("FizzBuzz");
        }else if i % 3 == 0{
            println!("Fizz");
        }else if i % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", i);
        }
    }
}

fn main() {
    for i in 1..=100 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
