pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut arr: Vec<i32> = (1..k + 1).collect();
    let mut i = k - 1;
    while i >= 0 {
        result.push(arr.clone());
        i = k - 1;
        while i >= 0 && arr[i as usize] == n - (k - i - 1) {
            i -= 1;
        }
        if i >= 0 {
            arr[i as usize] += 1;
            for j in i + 1..k {
                arr[j as usize] = arr[j as usize - 1] + 1;
            }
        }
    }
    result
}

fn main() {
    println!("{:?}", combine(4, 2));
    println!("{:?}", combine(1, 1));
}
