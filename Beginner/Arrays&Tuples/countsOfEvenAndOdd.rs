// 26. Create a program that counts even and odd numbers in an array.
fn main(){
    let numbers = [12,3,45,66,77,88,90,21,34];
    let mut even_count = 0;
    let mut odd_count = 0;
    let even = numbers.iter().filter(|x| *x %2 ==0);
    let odd = numbers.iter().filter(|x| *x %2 !=0);
    for _ in even{
        even_count +=1;
    }
    for _ in odd{
        odd_count +=1;
    }
    println!("The count of even numbers in the array is {}", even_count);
    println!("The count of odd numbers in the array is {}", odd_count);

}


fn main() {
    let numbers = [12, 3, 45, 66, 77, 88, 90, 21, 34];

    let even_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    let odd_count  = numbers.iter().filter(|&&x| x % 2 != 0).count();

    println!("Even count: {}", even_count);
    println!("Odd count: {}", odd_count);
}
