use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let opposite_bracket = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if opposite_bracket.contains_key(&c) {
            stack.push(c);
        } else {
            let stack_top = stack.pop();
            if stack_top == None {
                return false;
            }
            if c != *opposite_bracket.get(&stack_top.unwrap()).unwrap() {
                return false;
            }
        }
    }
    return if stack.len() != 0 { false } else { true };
}

fn main() {
    println!("{}", is_valid(String::from("()")));
    println!("{}", is_valid(String::from("()[]{}")));
    println!("{}", is_valid(String::from("(}")));
    println!("{}", is_valid(String::from("{[()]}")));
    println!("{}", is_valid(String::from("()[]{})")));
    println!("{}", is_valid(String::from("(")));
}
