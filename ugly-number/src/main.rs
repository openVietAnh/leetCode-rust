pub fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut val = n;
    while val % 2 == 0 {
        val /= 2;
    }
    while val % 3 == 0 {
        val /= 3;
    }
    while val % 5 == 0 {
        val /= 5;
    }
    val == 1
}

fn main() {
    println!("{}", is_ugly(6));
    println!("{}", is_ugly(2));
    println!("{}", is_ugly(3));
    println!("{}", is_ugly(5));
    println!("{}", is_ugly(14));
    println!("{}", is_ugly(1641249143));
}
