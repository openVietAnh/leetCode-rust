pub fn is_power_of_two(n: i32) -> bool {
    if n <= 0 || (n != 1 && n % 2 != 0) {
        false
    } else {
        let log_value = (n as f64).log2();
        println!("{}", log_value);
        log_value == log_value.ceil()
    }
}

fn main() {
    println!("{:?}", is_power_of_two(2147483646));
}
