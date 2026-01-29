// 48. Create a program that checks if a number is palindrome.

fn main(){
    let num = 121;
    let mut reversed = 0;
    let mut temp = num;

    while temp != 0{
        let rem = temp %10;
        reversed = reversed *10 + rem;
        temp /= 10;
    }

    if num == reversed{
        println!("{} is a palindrome number", num);
    }else{
        println!("{} is not a palindrome number", num);
    }
}