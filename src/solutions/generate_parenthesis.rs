use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n == 1 {
        return vec![String::from("()")];
    }
    let mut parenthesis: Vec<HashSet<String>> = vec![HashSet::new(); n as usize + 1];
    parenthesis[1].insert(String::from("()"));
    parenthesis[2].insert(String::from("()()"));
    parenthesis[2].insert(String::from("(())"));
    for i in 2..n as usize {
        let current_i = &parenthesis[i];
        for item in current_i.clone() {
            parenthesis[i + 1].insert("(".to_owned() + &item + ")");
            for j in 1..i + 1 {
                if i + j <= n as usize {
                    for j_item in parenthesis[j].clone() {
                        parenthesis[i + j].insert(j_item.clone() + &item);
                        parenthesis[i + j].insert(item.clone() + &j_item);
                    }
                }
            }
        }
    }
    println!("{:?}", parenthesis);
    parenthesis
        .get(n as usize)
        .unwrap()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}
