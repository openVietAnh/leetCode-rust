pub fn length_of_last_word(s: String) -> i32 {
    s.trim()
        .split(' ')
        .into_iter()
        .last()
        .unwrap()
        .len()
        .try_into()
        .unwrap()
}
