pub fn convert_to_title(column_number: i32) -> String {
    let mut number = column_number;
    let mut result = String::from("");
    while number > 0 {
        let value = number % 26;
        if value == 0 {
            result = String::from("Z") + &result;
            number -= 1;
        } else {
            result = char::from_u32((number % 26 + 64) as u32)
                .unwrap()
                .to_string()
                + &result;
        }
        number /= 26;
    }
    return result;
}
