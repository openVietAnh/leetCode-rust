use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut s = HashMap::new();
    for (index, item) in nums.into_iter().enumerate() {
        if s.contains_key(&item) && index - s.get(&item).unwrap() <= k.try_into().unwrap() {
            return true;
        }
        s.entry(item)
            .and_modify(|value| *value = index)
            .or_insert(index);
    }
    false
}

fn main() {
    println!("{:?}", contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    println!("{:?}", contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    println!("{:?}", contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
}
