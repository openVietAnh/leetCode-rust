use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut checked = HashSet::new();
    let mut number: u32 = n as u32;
    while number != 1 {
        number = number
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().pow(2))
            .sum();
        if checked.contains(&number) {
            return false;
        }
        checked.insert(number);
    }
    return true;
}
