// 50. Create a program that finds all prime numbers up to n (Sieve of Eratosthenes).

fn main() {
    let n = 50;

    if n < 2 {
        return;
    }

    let mut is_prime = vec![true; (n + 1) as usize];

    is_prime[0] = false;
    is_prime[1] = false;

    let limit = (n as f64).sqrt() as usize;

    for i in 2..=limit {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= n as usize {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    println!("Prime numbers up to {}:", n);
    for i in 2..=n as usize {
        if is_prime[i] {
            print!("{} ", i);
        }
    }
}
