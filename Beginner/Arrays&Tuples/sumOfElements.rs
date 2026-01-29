// 22. Create a program that finds the sum of all elements in an array.

fn main(){
    let numbers = [1,2,3,4,5];
    let sum = numbers.iter().sum::<i32>();
    println!("The sum of all elements in the array is {}", sum);
}