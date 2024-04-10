pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let (mut full, mut result, mut empty) = (num_bottles, 0, 0);
    while full > 0 {
        result += full;
        empty += full;
        full = empty / num_exchange;
        empty = empty % num_exchange;
    }
    return result;
}

fn main() {
    println!("{:?}", num_water_bottles(9, 3));
}
