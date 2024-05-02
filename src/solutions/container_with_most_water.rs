use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_value = min(height[0], height[1]);
    let mut current_optimal_solution = vec![(0, 1)];
    for i in 2..height.len() {
        for (l, r) in current_optimal_solution.clone() {
            let l_size = (i - l) as i32 * min(height[i], height[l]);
            let r_size = (i - r) as i32 * min(height[i], height[r]);
            if max(l_size, r_size) == max_value {
                if l_size > r_size {
                    current_optimal_solution.push((l, i));
                } else if r_size > l_size {
                    current_optimal_solution.push((r, i));
                } else {
                    current_optimal_solution.push((r, i));
                    current_optimal_solution.push((l, i));
                }
            } else if max(l_size, r_size) > max_value {
                max_value = max(l_size, r_size);
                if l_size > r_size {
                    current_optimal_solution = vec![(l, i)];
                } else if r_size > l_size {
                    current_optimal_solution = vec![(r, i)];
                } else {
                    current_optimal_solution = vec![(r, i), (l, i)];
                }
            }
        }
        println!("{:?}", current_optimal_solution);
    }
    return max_value.try_into().unwrap();
}
