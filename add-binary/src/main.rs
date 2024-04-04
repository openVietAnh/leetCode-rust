pub fn add_binary(a: String, b: String) -> String {
    let (mut num1, mut num2) = (a.chars().rev(), b.chars().rev());
    let mut result = String::from("");
    let mut remember = 0;
    loop {
        let char1 = num1.next();
        let char2 = num2.next();
        if char1.is_none() && char2.is_none() {
            break;
        } else {
            let value = if char1.is_some() && char2.is_some() {
                char1.unwrap().to_digit(2).unwrap() + char2.unwrap().to_digit(2).unwrap() + remember
            } else if char1.is_none() {
                char2.unwrap().to_digit(2).unwrap() + remember
            } else {
                char1.unwrap().to_digit(2).unwrap() + remember
            };
            remember = value / 2;
            result = (value % 2).to_string() + &result;
        }
    }
    return if remember == 0 {
        result
    } else {
        String::from("1") + &result
    };
}

fn main() {
    println!("{}", add_binary(String::from("11"), String::from("1")));
    println!("{}", add_binary(String::from("1010"), String::from("1011")));
}
