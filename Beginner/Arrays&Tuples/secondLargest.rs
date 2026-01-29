// 29. Write a program that finds the second largest number in an array.


fn main(){
    let numbers = [3,4,5,6,7,8,9,1,2];
    let mut largest = i32::MIN;
    let mut second_largest = i32::MIN;
    for &num in &numbers{
        if num > largest{
            second_largest = largest;
            largest = num;
        } else if num > second_largest && num != largest{
            second_largest = num;
        }
    }
    if second_largest == i32::MIN{
        println!("There is no second largest element in the array");
    } else {
        println!("The second largest element in the array is {}", second_largest);

    }
}