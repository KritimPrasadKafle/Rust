// 47. Write a program that reverses a number.

fn main(){
    let mut num = 12345;
    let mut result = 0;
    while num != 0{
        let rem = num %10;
        result = result * 10 + rem;
        num /= 10;
    }
    println!("Reversed number is: {}", result);
}

fn reverse_number(mut num: i32) -> i32 {
    let sign = num.signum();
    num = num.abs();

    let mut result = 0;
    while num != 0 {
        result = result * 10 + num % 10;
        num /= 10;
    }

    result * sign
}
