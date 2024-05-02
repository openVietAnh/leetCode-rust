pub fn is_palindrome(s: String) -> bool {
    let filtered: String = s
        .to_ascii_lowercase()
        .to_string()
        .chars()
        .into_iter()
        .filter(|c| c.is_alphanumeric())
        .collect();
    return filtered == filtered.chars().into_iter().rev().collect::<String>();
}
