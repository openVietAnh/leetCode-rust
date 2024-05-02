use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let chars1 = s.chars().collect::<Vec<char>>();
    let chars2 = t.chars().collect::<Vec<char>>();
    let mut mapping = HashMap::new();
    let mut mapped = HashMap::new();
    for i in 0..s.len() {
        if mapping.contains_key(&chars1[i]) {
            if chars2[i] != *mapping.get(&chars1[i]).unwrap() {
                return false;
            }
        } else if mapped.contains_key(&chars2[i]) {
            return false;
        } else {
            mapping.insert(chars1[i], chars2[i]);
            mapped.insert(chars2[i], chars1[i]);
        }
    }
    true
}
