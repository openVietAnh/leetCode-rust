pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = (nums.len() * (nums.len() + 1) / 2) as i32;
    for item in nums {
        sum -= item;
    }
    sum
}

fn main() {
    println!("{}", missing_number(vec![0, 1, 3]));
}
