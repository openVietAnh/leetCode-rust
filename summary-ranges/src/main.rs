pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut index = 0;
    while index < nums.len() {
        let mut j = index;
        while j + 1 < nums.len() && nums[j + 1] == nums[j] + 1 {
            j += 1;
        }
        if j > index {
            result.push(nums[index].to_string() + "->" + &nums[j].to_string());
        } else {
            result.push(nums[index].to_string());
        }
        index = j + 1;
    }
    result
}

fn main() {
    println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7]));
    println!("{:?}", summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]));
}
