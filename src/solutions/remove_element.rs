pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    while index < nums.len() {
        while index < nums.len() && nums[index] == val {
            nums.remove(index);
        }
        index += 1
    }
    return nums.len().try_into().unwrap();
}
