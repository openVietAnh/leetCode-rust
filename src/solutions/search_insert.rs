pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let result = nums.binary_search(&target);
    match result {
        Err(index) => index.try_into().unwrap(),
        Ok(index) => index.try_into().unwrap(),
    }
}
