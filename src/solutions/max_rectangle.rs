pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let mut column = vec![0; matrix[0].len()];
    let mut result = 0;
    for i in 0..matrix.len() {
        for (index, item) in matrix[i].clone().into_iter().enumerate() {
            if item == '1' {
                column[index] += 1;
            } else {
                column[index] = 0;
            }
        }
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '0' {
                continue;
            } else {
                let mut count = 1;
                let mut min_col = column[j];
                result = std::cmp::max(count * min_col, result);
                for k in (j + 1)..matrix[i].len() {
                    if matrix[i][k] == '0' {
                        break;
                    } else {
                        count += 1;
                        min_col = std::cmp::min(column[k], min_col);
                        result = std::cmp::max(count * min_col, result);
                    }
                }
            }
        }
    }
    return result;
}
