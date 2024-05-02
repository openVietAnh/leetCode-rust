struct NumArray {
    nums: Vec<i32>,
    sum_prefix: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![];
        let mut moving_sum = 0;
        for item in &nums {
            moving_sum += item;
            sum.push(moving_sum);
        }
        let new_obj = NumArray {
            nums,
            sum_prefix: sum,
        };
        new_obj
    }
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == right {
            self.nums[left as usize]
        } else if left == 0 {
            self.sum_prefix[right as usize]
        } else {
            self.sum_prefix[right as usize] - self.sum_prefix[(left - 1) as usize]
        }
    }
}
