// 42. Create a program that prints even numbers from 1 to 50.

fn main(){
    for i in 0..=50{
        if i %2 == 0{
            println!("{}", i);
        }else{
            continue;
        }
    }
}

fn main() {
    for i in (1..=50).filter(|n| n % 2 == 0) {
        println!("{}", i);
    }
}
