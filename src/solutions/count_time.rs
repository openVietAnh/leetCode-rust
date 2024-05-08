pub fn count_time(time: String) -> i32 {
    let (mut valid_hours, mut valid_minutes) = (1, 1);
    let chars: Vec<char> = time.chars().collect();
    if chars[0] == '?' {
        valid_hours = 3;
    }
    if chars[1] == '?' {
        if chars[0] == '?' {
            valid_hours = 24; 
        } else if chars[0] == '2' {
            valid_hours = 4;
        } else {
            valid_hours = 10;
        }
    }
    if chars[3] == '?' {
        valid_minutes = 6;
    }
    if chars[4] == '?' {
        if chars[3] == '?' {
            valid_minutes = 60; 
        } else {
            valid_minutes = 10;
        }
    }
    valid_hours * valid_minutes
}