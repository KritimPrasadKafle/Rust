// 23. Write a program that finds the maximum element in an array.

fn main(){
    let numbers = [3,2,4,5,10,1,2];
    let max = numbers.iter().max();
    match max{
        Some(&value) => println!("The maximum element in the array is {}", value),

        None => println!("The array is empty"),
    }
}