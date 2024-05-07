pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut result = 0;
    let mut chosen = 0;
    box_types.sort_by(|a, b| b[1].partial_cmp(&a[1]).unwrap());
    for item in box_types {
        if chosen + item[0] >= truck_size {
            return result + item[1] * (truck_size - chosen);
        } else {
            chosen += item[0];
            result += item[0] * item[1];
        }
    }
    result
}
