// 32. Create a program that rotates an array to the left by n positions.

fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6, 7];
    let n = 3;

    let len = numbers.len();
    let n = n % len;

    rotate_left(&mut numbers, n);

    println!(
        "The array after rotating left by {} positions is {:?}",
        n, numbers
    );
}

fn rotate_left<T>(arr: &mut [T], n: usize) {
    if n == 0 || n >= arr.len() {
        return;
    }

    arr[..n].reverse();
    arr[n..].reverse();
    arr.reverse();
}
