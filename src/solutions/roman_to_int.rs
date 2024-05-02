use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    let char_to_value: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut previous_char: char = ' ';
    for current_char in s.chars() {
        result += char_to_value.get(&current_char).unwrap();
        if previous_char != ' ' {
            if previous_char == 'I' {
                if current_char == 'V' || current_char == 'X' {
                    result -= char_to_value.get(&previous_char).unwrap() * 2;
                }
            }
            if previous_char == 'X' {
                if current_char == 'L' || current_char == 'C' {
                    result -= char_to_value.get(&previous_char).unwrap() * 2;
                }
            }
            if previous_char == 'C' {
                if current_char == 'D' || current_char == 'M' {
                    result -= char_to_value.get(&previous_char).unwrap() * 2;
                }
            }
        }
        previous_char = current_char;
    }
    return result;
}
