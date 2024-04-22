pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut vals = nums.clone();
    vals.sort();
    let mut result: Vec<Vec<i32>> = vec![];
    let mut previous_i = 0;
    for i in 0..vals.len() - 2 {
        if i != 0 && vals[i] == previous_i {
            continue;
        }
        let mut previous_j = 0;
        for j in i + 1..vals.len() - 1 {
            if j != i + 1 && vals[j] == previous_j {
                continue;
            }
            let find = vals[j + 1..vals.len()].binary_search(&-(vals[i] + vals[j]));
            if find.is_ok() {
                result.push(vec![vals[i], vals[j], -(vals[i] + vals[j])]);
            }
            previous_j = vals[j];
        }
        previous_i = vals[i];
    }
    result
}

fn main() {
    println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
