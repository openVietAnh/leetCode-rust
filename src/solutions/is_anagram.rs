use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut counter1 = HashMap::new();
    let mut counter2 = HashMap::new();
    for c in s.chars() {
        counter1
            .entry(c)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }
    for c in t.chars() {
        counter2
            .entry(c)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }
    if counter1.keys().len() != counter2.keys().len() {
        return false;
    }
    for (key, value) in counter1.iter() {
        if !counter2.contains_key(key) {
            return false;
        }
        if counter2.get(key).unwrap() != value {
            return false;
        }
    }
    true
}
