pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n: i32 = nums.len() as i32; // Because len() returns usize
    let mut sum: i32 = (n * (n + 1)) / 2; // Sum of n natural numbers

    for num in nums {
        sum = sum - num;
    }

    return sum;
}