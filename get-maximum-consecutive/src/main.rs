pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
    let mut c = coins.clone();
    c.sort();
    let mut result = 1;
    for item in c {
        if item <= result {
            result += item;
        }
    }
    result
}

fn main() {
    println!("{}", get_maximum_consecutive(vec![1, 1, 1, 4]));
    println!("{}", get_maximum_consecutive(vec![1, 3]));
    println!("{}", get_maximum_consecutive(vec![1, 4, 10, 3, 1]));
}
