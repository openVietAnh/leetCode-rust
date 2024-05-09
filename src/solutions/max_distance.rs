use std::collections::HashMap;

pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut min_color: HashMap<i32, usize> = HashMap::new();
    let mut result = 0;
    for (index, item) in colors.into_iter().enumerate() {
        if !min_color.contains_key(&item) {
            min_color.insert(item, index);
        }
        for key in min_color.keys() {
            if *key != item {
                result = std::cmp::max(result, (index - min_color.get(key).unwrap()) as i32);
            }
        }
    }
    result
}
