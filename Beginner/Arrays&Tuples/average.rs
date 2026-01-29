// 25. Write a program that calculates the average of numbers in an array.
fn main(){
    let numbers = [10,15,20,25,30];
    let sum = numbers.iter().sum::<i32>();
    let count = numbers.len() as f32;
    let average = sum as f32 / count;
    println!("The average of the numbers in the array is {}", average);
}