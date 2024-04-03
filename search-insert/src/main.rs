pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let result = nums.binary_search(&target);
    match result {
        Err(index) => { index.try_into().unwrap() }
        Ok(index) => { index.try_into().unwrap() }
    }
}

fn main() {
    println!("{:?}", search_insert(vec![1, 2, 3, 4], 3));
    println!("{:?}", search_insert(vec![1, 3, 5, 6], 4));
    println!("{:?}", search_insert(vec![1, 3, 5, 6], 7));
}
