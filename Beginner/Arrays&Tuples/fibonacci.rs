// 54. Create a program that prints Fibonacci series up to n terms.


fn main() {
    let n = 10;
    let mut a = 0;
    let mut b = 1;

    println!("Fibonacci series up to {} terms:", n);

    for _ in 0..n {
        print!("{} ", a);  // print current term
        let next = a + b;  // calculate next term
        a = b;             // move forward
        b = next;
    }
    println!(); // newline at the end
}
