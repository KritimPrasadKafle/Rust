// 21. Write a program that creates an array of 5 integers and prints each element.

fn main(){
    let numbers = [10, 20, 30, 40, 50];
    for ii in 0..numbers.len(){
        println!("Element at index {}: {}", ii, numbers[ii]);
    }

    for (index, value) in numbers.iter().enumerate(){
        println!("Element at index {}: {}", index, value);
    }
}