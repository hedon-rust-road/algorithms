pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    // [0, k] 只包含非重复元素
    let mut k: usize = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[k] {
            k += 1;
            if i != k {
                nums.swap(k, i);
            }
        }
    }
    return (k + 1) as i32;
}
