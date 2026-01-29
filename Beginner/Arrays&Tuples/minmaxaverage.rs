// 31. Write a function that returns a tuple of (min, max, average) from an array.

fn main() {
    let nums = [1, 2, 3, 4, 54];

    let (min, max, avg) = min_max_avg(&nums);

    println!("Min: {}, Max: {}, Average: {}", min, max, avg);
}

fn min_max_avg(nums: &[i32]) -> (i32, i32, f64) {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let sum: i32 = nums.iter().sum();
    let average = sum as f64 / nums.len() as f64;

    (min, max, average)
}
