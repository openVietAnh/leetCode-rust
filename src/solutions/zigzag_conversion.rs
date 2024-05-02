use std::i16;

pub fn convert(s: String, num_rows: i32) -> String {
    let mut rows: Vec<Vec<char>> = vec![vec![]; num_rows as usize];
    let (mut direction, mut index) = (1 as i16, 0 as i16);
    for i in s.chars() {
        rows[index as usize].push(i);
        if num_rows != 1 {
            if (index == 0 && direction == -1) || (index == (num_rows - 1) as i16 && direction == 1)
            {
                direction *= -1;
            }
            index += direction;
        }
    }
    let mut result = String::from("");
    for row in rows {
        result += &row.into_iter().collect::<String>();
    }
    return result;
}
