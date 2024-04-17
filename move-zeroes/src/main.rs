pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut end = nums.len() - 1;
    while nums[end] == 0 {
        end -= 1;
    }
    let mut ind = 0;
    while ind < end {
        if nums[ind] == 0 {
            nums.remove(ind);
            nums.push(0);
            end -= 1;
        } else {
            ind += 1;
        }
    }
}

fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut v);
    println!("{:?}", v);
}
