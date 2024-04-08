pub fn title_to_number(column_title: String) -> i32 {
    let (mut pow, mut result) = (1 as u32, 0);
    for c in column_title.chars().rev() {
        result += (c as u8 - 64) as u32 * pow;
        pow *= 26;
    }
    return result.try_into().unwrap();
}

fn main() {
    println!("{}", title_to_number(String::from("A")));
    println!("{}", title_to_number(String::from("Z")));
    println!("{}", title_to_number(String::from("AB")));
}
