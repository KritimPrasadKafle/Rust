// 51. Write a program that calculates GCD of two numbers using Euclidean algorithm.

fn main() {
    let a = 48;
    let b = 18;
    let gcd = euclidean_gcd(a, b);
    println!("GCD of {} and {} is {}", a, b, gcd);
}

fn euclidean_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}