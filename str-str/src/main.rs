pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        index.try_into().unwrap()
    }
    else { -1 }
}

fn main() {
    println!("{}", str_str(String::from("sadbutsad"), String::from("sad")));
    println!("{}", str_str(String::from("sadbutsad"), String::from("adb")));
    println!("{}", str_str(String::from("leetcode"), String::from("leeto")));
}
