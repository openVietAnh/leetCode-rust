pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for i in 0..mat.len() {
        result += mat[i][i];
        result += mat[i][mat.len() - i - 1]
    }
    return if mat.len() % 2 == 1 {
        result - mat[mat.len() / 2][mat.len() / 2]
    } else {
        result
    };
}

