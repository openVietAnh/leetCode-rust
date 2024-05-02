pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut index1, mut index2) = (0, 0);
    while index2 < nums2.len() {
        if index1 < m as usize + index2 && nums2[index2] > nums1[index1] {
            index1 += 1;
        } else {
            nums1.insert(index1, nums2[index2]);
            nums1.pop();
            index2 += 1;
        }
    }
}
