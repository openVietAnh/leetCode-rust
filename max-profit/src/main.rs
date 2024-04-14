pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut found = false;
    for i in 0..prices.len() - 1 {
        if prices[i] < prices[i + 1] {
            found = true;
            break;
        }
    }
    if !found {
        return 0;
    }
    let mut enumerate = prices.clone().into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    enumerate.sort_by(|a, b| a.1.cmp(&b.1));
    let mut result = 0; 
    for i in 0..enumerate.len() {
        let mut ind = enumerate.len() - 1;
        loop {
            if enumerate[ind].0 > enumerate[i].0 {
                if enumerate[ind].1 - enumerate[i].1 > result {
                    result = enumerate[ind].1 - enumerate[i].1;
                }
                break;
            } 
            else if ind > i {
                ind -= 1;
            } else {
                break;
            }
        }
    }
    result
}

fn main() {
    println!("{:?}", max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{:?}", max_profit(vec![7, 6, 4, 3, 1]));
    println!("{:?}", max_profit(vec![2, 4, 1]));
}
