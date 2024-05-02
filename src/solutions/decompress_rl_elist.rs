pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for i in (0..nums.len()).step_by(2) {
        result.append(&mut [nums[i + 1]].repeat(nums[i] as usize))
    }
    result
}
