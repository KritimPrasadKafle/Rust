// 44. Create a program that calculates factorial of a number using a loop.

// fn main(){
//     let number = 5;
//     let mut factorial = 1;

//     for i in 1..=number{
//         factorial *= i;
//     }

//     println!("Factorial of {} is {}", number, factorial);
// }

fn main(){
    let number = 5;
    let factorial: i32 = (1..=number).product();
    println!("Factorial of {} is {}", number, factorial);
}