pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min = nums.len() + 1;
    let mut left = 0;
    let mut sum = 0;

    for (right, num) in nums.iter().enumerate() {
        sum += num;

        // 满足条件后开始收缩窗口
        while sum >= target {
            min = min.min(right - left + 1);
            sum -= nums[left];
            left += 1;
        }
    }

    if min == nums.len() + 1 { 0 } else { min as i32 }
}
