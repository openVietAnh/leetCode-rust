pub fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        false
    } else {
        let log_value = (n as f64).log(3.0);
        (log_value - log_value.ceil()).abs() < 1e-10
    }
}

fn main() {
    println!("{:?}", is_power_of_three(243));
    println!("{:?}", is_power_of_three(8));
}
