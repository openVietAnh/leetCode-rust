use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut mapping: HashMap<char, String> = HashMap::new();
    let mut mapped = HashMap::new();
    let words: Vec<String> = s.split_ascii_whitespace().map(|s| s.to_string()).collect();
    let chars: Vec<char> = pattern.chars().collect();
    if words.len() != chars.len() {
        return false;
    }
    for i in 0..chars.len() {
        if mapping.contains_key(&chars[i]) {
            if *mapping.get(&chars[i]).unwrap() != words[i] {
                return false;
            }
        }
        else {
            if mapped.contains_key(&words[i]) {
                return false;
            }
            mapping.insert(chars[i], words[i].clone());
            mapped.insert(words[i].clone(), chars[i]);
        }
    }
    true
}

fn main() {
    println!("{}", word_pattern(String::from("abbc"), String::from("dog word word cat")));
}
