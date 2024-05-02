pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        vec![vec![1]]
    } else if num_rows == 2 {
        vec![vec![1], vec![1, 1]]
    } else {
        let mut result = vec![vec![1], vec![1, 1]];
        for i in 2..num_rows {
            let mut new_row: Vec<i32> = vec![1];
            let last_row = result.last().unwrap();
            for j in 0..last_row.len() - 1 {
                new_row.push(last_row[j] + last_row[j + 1]);
            }
            new_row.push(1);
            result.push(new_row.clone());
        }
        result
    }
}
