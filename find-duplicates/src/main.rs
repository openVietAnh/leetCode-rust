use std::collections::HashSet;

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut s: HashSet<usize> =
        HashSet::from_iter((1..(nums.len() + 1)).into_iter().collect::<Vec<usize>>());
    let mut result = vec![];
    for item in nums {
        if s.contains(&(item as usize)) {
            s.remove(&(item as usize));
        } else {
            result.push(item);
        }
    }
    return result;
}

fn main() {
    println!("{:?}", find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]));
}
