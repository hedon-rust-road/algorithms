pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut right: usize = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums.swap(right, i);
            right += 1;
        }
    }
    return right as i32;
}
