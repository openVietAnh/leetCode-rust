use std::collections::HashSet;

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut result = 0;
    for value in left..(right + 1) {
        let (mut num, mut count) = (value, 0);
        while num > 0 {
            count += if num % 2 == 1 { 1 } else { 0 };
            num /= 2;
        }
        if HashSet::from([2, 3, 5, 7, 11, 13, 17, 19]).contains(&count) {
            result += 1;
        };
    }
    return result;
}
