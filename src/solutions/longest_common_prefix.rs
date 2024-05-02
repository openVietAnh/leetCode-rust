pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let min_length = strs.iter().map(|s| s.len()).min().unwrap();
    let mut common = "";
    for index in 0..(min_length + 1) {
        let substring = &strs[0][0..index];
        if !strs.iter().all(|s| s.starts_with(substring)) {
            return common.to_string();
        } else {
            common = substring;
        }
    }
    return common.to_string();
}
