pub fn add_digits(num: i32) -> i32 {
    if num <= 9 {
        return num;
    }
    if num % 9 == 0 {
        num - ((num / 9 - 1) * 9)
    } else {
        num - ((num / 9) * 9)
    }
}

fn main() {
    println!("{}", add_digits(38));
}
