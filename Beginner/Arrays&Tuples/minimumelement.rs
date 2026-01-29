// 24. Create a program that finds the minimum element in an array.

fn main(){
    let numbers = [3,2,4,5,10,1,2];
    let min = numbers.iter().min();
    match min{
        Some(&value) => println!("The minimum element in the array is {}", value),

        None => println!("The array is empty"),
    }
}