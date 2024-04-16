pub struct Solution;

impl Solution {
    pub fn isBadVersion(v: i32) -> bool {
        v == 2147483647
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let delta = 1.0 + 8.0 * n as f64;
        let solution = (-1.0 + delta.sqrt()) / 2.0;
        let max_interval = solution.ceil() as i64;
        println!("{}", max_interval);
        let mut interval = max_interval;
        let mut version: i64 = max_interval;
        let mut before = 1;
        loop {
            if !Solution::isBadVersion(version as i32) {
                interval -= 1;
                before = version;
                version = std::cmp::min(n as i64, version + interval);
            } else {
                for i in before..(version + 1) {
                    if Solution::isBadVersion(i as i32) {
                        return i as i32;
                    }
                }
            }
        }
    }
}

fn main() {
    let s = Solution;
    println!("{}", s.first_bad_version(2147483647));
}
