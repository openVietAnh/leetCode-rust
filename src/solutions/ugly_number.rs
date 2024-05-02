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
