pub fn min_operations(s: String) -> i32 {
    let (mut count1, mut count2) = (0, 0);
    for (index, c) in s.chars().enumerate() {
        if index % 2 == 0 {
            if c != '0' {
                count1 += 1;
            }
            if c != '1' {
                count2 += 1;
            }
        } else {
            if c != '0' {
                count2 += 1;
            }
            if c != '1' {
                count1 += 1;
            }
        }
    }
    std::cmp::min(count1, count2)
}
