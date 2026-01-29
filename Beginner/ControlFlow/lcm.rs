// 52. Create a program that calculates LCM of two numbers.
fn main() {
    let a = 12;
    let b = 15;
    let lcm = calculate_lcm(a, b);
    println!("LCM of {} and {} is {}", a, b, lcm);
}

fn calculate_lcm(a: i32, b: i32) -> i32 {
    (a * b) / euclidean_gcd(a, b)
}

fn euclidean_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}