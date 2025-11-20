pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    // [0, k] 满足条件，重复元素不超过 2 个
    let mut k: usize = 1;
    for i in 2..nums.len() {
        if nums[i] != nums[k - 1] {
            k += 1;
            if k != i {
                nums.swap(k, i);
            }
        }
    }
    return (k + 1) as i32;
}
