pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index = 0;
    while index < nums.len() {
        while index < nums.len() - 1 && nums[index] == nums[index + 1] {
            nums.remove(index);
        }
        index += 1;
    }
    return nums.len().try_into().unwrap();
}

fn main() {
    println!("{}", remove_duplicates(&mut vec![1, 1, 2]));
    println!(
        "{}",
        remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
    );
}
