// 30. Create a program that removes duplicates from a sorted array.

// fn main(){
//     let numbers = [1,2,2,3,4,4,5];
//     let mut unique_numbers = Vec::new();
//     for &num in numbers.iter(){
//         if !unique_numbers.contains(&num){
//             unique_numbers.push(num);
//         }
//     }
//     println!("Array after removing duplicates: {:?}", unique_numbers);
// }

use std::collections::HashSet;
fn main(){
    let num = [1,2,3,4,4,3,1,2,5,43];
    let mut set = HashSet::new();
    for &n in num.iter(){
        set.insert(n);
    }
    println!("Array after removing duplicates: {:?}", set);

}