// 5. Write a program that swaps two variables using tuple destructuring.
fn main() {
    let mut a = 10;
    let mut b = 20;
    
    println!("Before: a = {}, b = {}", a, b);
    
    (a, b) = (b, a);  // ← Swap using tuple destructuring
    
    println!("After: a = {}, b = {}", a, b);
}