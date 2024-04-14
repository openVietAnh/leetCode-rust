pub fn get_row(row_index: i32) -> Vec<i32> {
    if row_index == 0 {
        vec![1]
    } else if row_index == 1{
        vec![1, 1]
    } else {
        let mut result = vec![1, 1];
        for i in 2..row_index + 1 {
            let old_len = result.len();
            for j in 0..old_len - 1 {
                result.push(result[j] + result[j + 1]);
            }
            result.splice(1..(i as usize), []);
            result.push(1);
        }
        result
    }
}

fn main() {
    println!("{:?}", get_row(1));
    println!("{:?}", get_row(2));
    println!("{:?}", get_row(3));
    println!("{:?}", get_row(4));
    println!("{:?}", get_row(5));
}
