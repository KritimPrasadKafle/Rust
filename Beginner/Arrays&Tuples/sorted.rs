// 28. Create a program that checks if an array is sorted.

fn main(){
    let numbers = [1,2,3,4,5,2];
    for i in 0..numbers.len()-1{
        if numbers[i] > numbers[i+1]{
            println!("The array is not sorted");
            return;
        }
    }
    println!("The array is sorted");
}