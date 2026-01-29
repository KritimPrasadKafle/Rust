// 53. Write a program that checks if a number is Armstrong number.



fn main(){
    let num = 153;
    let mut sum = 0;
    let mut temp = num;
    let digits = num.to_string().len() as u32;

    while temp != 0 {
        let rem: u32 = (temp % 10) as u32; // <-- explicitly u32
        sum += rem.pow(digits);
        temp /= 10;
    }
    
    if sum == num {
        println!("{} is an Armstrong number", num);
    } else {
        println!("{} is not an Armstrong number", num);
    }
}
