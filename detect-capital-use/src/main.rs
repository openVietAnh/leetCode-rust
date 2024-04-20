pub fn detect_capital_use(word: String) -> bool {
    if word.to_ascii_lowercase().to_string() == word 
    || word.to_ascii_uppercase().to_string() == word {
        true
    } else {
        let mut iter = word.chars();
        iter.next();
        while let Some(c) = iter.next() {
            if c.is_ascii_uppercase() {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("{}", detect_capital_use(String::from("Google")));
    println!("{}", detect_capital_use(String::from("USA")));
    println!("{}", detect_capital_use(String::from("leetcode")));
    println!("{}", detect_capital_use(String::from("fLaG")));
}
