use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for item in nums {
        count
            .entry(item)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }
    *count
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap()
}

fn main() {
    println!("{}", majority_element(vec![3, 2, 3]));
    println!("{}", majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}
