use std::collections::HashMap;

pub fn tuple_same_product(mut nums: Vec<i32>) -> i32 {
    let mut prod_count: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if let Some(count) = prod_count.get(&((nums[i] * nums[j]) as i32)) {
                result += count * 8;
            }
            prod_count
                .entry((nums[i] * nums[j]) as i32)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }
    result
}
