pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut r_sum = 0;
    for item in nums {
        result.push(r_sum + item);
        r_sum += item;
    }
    return result;
}

fn main() {
    println!("Hello, world!");
}
