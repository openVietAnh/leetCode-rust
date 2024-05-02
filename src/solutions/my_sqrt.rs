pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }
    let (mut start, mut end) = (1, x / 2);
    let mut middle = (x / 2 + 1) / 2;
    loop {
        if start >= end {
            let min_val = std::cmp::min(start, end);
            if x / min_val < min_val {
                println!("here");
                return middle;
            } else {
                return min_val;
            }
        }
        middle = (start + end) / 2;
        if x / middle == middle {
            return middle;
        }
        if x / middle < middle {
            end = middle - 1;
        } else {
            if x / (middle + 1) < (middle + 1) {
                return middle;
            }
            start = middle + 1;
        }
    }
}
