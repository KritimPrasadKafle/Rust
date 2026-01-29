// 46. Create a program that finds the sum of digits of a number.

fn main(){

    let mut _sum = 0;
    let num = 123;

    for i in num.to_string().chars(){
        _sum += i.to_digit(10).unwrap();
        
    }
    println!("Current sum: {}", _sum);

}

fn main() {
    let mut num = 123;
    let mut sum = 0;

    while num > 0 {
        sum += num % 10;
        num /= 10;
    }

    println!("Sum of digits: {}", sum);
}
