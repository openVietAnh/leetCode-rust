pub fn climb_stairs(n: i32) -> i32 {
    let mut result: Vec<i32> = vec![0; n as usize + 1];
    result[0] = 1;
    for i in 0..n as usize {
        if i + 1 <= n.try_into().unwrap() {
            result[i + 1] += result[i];
        }
        if i + 2 <= n.try_into().unwrap() {
            result[i + 2] += result[i];
        }
    }
    return result[n as usize];
}
