use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut count: HashMap<u64, usize> = HashMap::new();
    let mut result: usize = 0;
    for item in dominoes {
        let mut domino = item.clone();
        if domino[0] > domino[1] {
            (domino[0], domino[1]) = (domino[1], domino[0]);
        }
        println!("{:?}", domino);
        let mut hasher = DefaultHasher::new();
        domino.hash(&mut hasher);
        println!("{}", hasher.finish());
        if let Some(value) = count.get(&hasher.finish()) {
            result += value;
        }
        count
            .entry(hasher.finish())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    result as i32
}

fn main() {
    println!(
        "{}",
        num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4],])
    );
}
