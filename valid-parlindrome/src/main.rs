pub fn is_palindrome(s: String) -> bool {
    let filtered: String = s
        .to_ascii_lowercase()
        .to_string()
        .chars()
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect();
    return filtered == filtered.chars().into_iter().rev().collect::<String>();
}

fn main() {
    println!("{}", is_palindrome(String::from("A man, a plan, a canal: Panama")));
}
