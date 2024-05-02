pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        index.try_into().unwrap()
    } else {
        -1
    }
}
