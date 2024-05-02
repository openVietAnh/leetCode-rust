pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    let mut even_ind = 0;
    let mut odd_ind = 1;
    while even_ind < nums.len() && odd_ind < nums.len() {
        while even_ind < nums.len() && nums[even_ind] % 2 == 0 {
            even_ind += 2;
        }
        while odd_ind < nums.len() && nums[odd_ind] % 2 != 0 {
            odd_ind += 2;
        }
        if even_ind < nums.len() && odd_ind < nums.len() {
            (nums[even_ind], nums[odd_ind]) = (nums[odd_ind], nums[even_ind]);
        }
    }
    nums
}
