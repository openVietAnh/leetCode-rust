use std::collections::HashSet;

pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<(i32, HashSet<i32>)> = (0..nums.len()).map(|_| (1, HashSet::new())).collect();
    let mut result = 1;
    for i in 0..nums.len() {
        for j in 0..i {
            if dp[i].0 == 1 {
                result = std::cmp::max(result, 2);
                dp[i] = (2, HashSet::from([nums[i] - nums[j]]));
            } else if dp[i].0 == 2 {
                dp[i].1.insert(nums[i] - nums[j]);
            }
            if dp[j].1.contains(&(nums[i] - nums[j])) {
                if dp[j].0 + 1 > dp[i].0 {
                    dp[i] = (dp[j].0 + 1, HashSet::from([nums[i] - nums[j]]));
                    result = std::cmp::max(result, dp[i].0);
                } else if dp[j].0 + 1 == dp[i].0 {
                    dp[i].1.insert(nums[i] - nums[j]);
                }
            }
        }
    }
    for i in 0..nums.len() {
        println!("{} {:?}", nums[i], dp[i]);
    }
    result
}
