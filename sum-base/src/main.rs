pub fn sum_base(n: i32, k: i32) -> i32 {
    let mut val = n;
    let mut result = 0;
    while val != 0 {
        result += val % k;
        val /= k;
    }
    result
}

fn main() {
    println!("{}", sum_base(39, 6));
}
