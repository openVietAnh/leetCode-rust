pub fn hamming_weight(n: i32) -> i32 {
    let (mut count, mut value) = (0, n);
    while value != 0 {
        if value % 2 == 1 {
            count += 1;
        }
        value /= 2;
    }
    return count;
}

fn main() {
    println!("{}", hamming_weight(11));
}
