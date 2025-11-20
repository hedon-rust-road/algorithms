pub fn move_zeroes(nums: &mut Vec<i32>) {
    // [0, right] 为非 0
    let mut right = -1;
    let n = nums.len();
    for i in 0..n {
        if nums[i] != 0 {
            right += 1;
            nums.swap(right as usize, i);
        }
    }
}
