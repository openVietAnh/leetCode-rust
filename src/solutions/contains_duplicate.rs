use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut s = HashSet::new();
    for item in nums {
        if s.contains(&item) {
            return true;
        }
        s.insert(item);
    }
    false
}
