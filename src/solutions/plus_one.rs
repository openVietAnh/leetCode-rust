pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = digits.clone();
    let mut remember = 0;
    for i in (0..result.len()).rev() {
        let value = result[i] + if i == result.len() - 1 { 1 } else { 0 } + remember;
        remember = value / 10;
        result[i] = value % 10;
    }
    if remember == 1 {
        [vec![1], result].concat()
    } else {
        result
    }
}
