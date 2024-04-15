pub fn reverse_bits(x: u32) -> u32 {
    let (mut result, mut pow, mut num) = (0, 31, x);
    while num != 0 {
        result += (num % 2 * 2).pow(pow);
        num /= 2;
        pow -= 1;
    }
    return result;
}

fn main() {
    println!("{:?}", reverse_bits(43261596));
}
