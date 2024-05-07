pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut ind = 0;
    while ind < nums.len() && nums[ind] < 0 && ind < k as usize {
        nums[ind] *= -1;
        ind += 1;
    }
    if (ind == nums.len()) {
        if (k as usize - ind) % 2 == 1 {
            nums[ind - 1] *= -1;
            return nums.into_iter().sum();
        }
    }
    if ind < k as usize && (k as usize - ind) % 2 == 1 {
        if ind == 0 || nums[ind] == 0 || (ind > 0 && nums[ind] < nums[ind - 1]) { 
            nums[ind] *= -1; 
        } else {
            nums[ind - 1] *= -1;
        }
    }
    nums.into_iter().sum()
}