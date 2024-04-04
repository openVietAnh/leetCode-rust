pub fn length_of_last_word(s: String) -> i32 {
    s.trim()
        .split(' ')
        .into_iter()
        .last()
        .unwrap()
        .len()
        .try_into()
        .unwrap()
}

fn main() {
    println!("{}", length_of_last_word(String::from("Hello World")));
    println!(
        "{}",
        length_of_last_word(String::from("   fly me   to   the moon "))
    );
    println!(
        "{}",
        length_of_last_word(String::from("luffy is still joyboy"))
    );
}
