pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let nums = &mut nums.clone();
    let mut left: usize = 0;
    let mut right = nums.len() - 1;
    let target_index = nums.len() - k as usize;

    while left <= right {
        let pivot_index = left;
        let pivot_val = nums[pivot_index];

        // [left, lt-1] < pivot_val
        // [lt, gt] = pivot_val
        // [gt+1, right] > pivot_val
        let mut lt = left;
        let mut gt = right;
        let mut i = lt + 1;
        while i <= gt {
            if nums[i] < pivot_val {
                // 小于的放左边
                nums.swap(i, lt);
                i += 1;
                lt += 1;
            } else if nums[i] > pivot_val {
                nums.swap(i, gt);
                gt -= 1;
            } else {
                i += 1;
            }
        }

        if target_index >= lt && target_index <= gt {
            return pivot_val;
        } else if target_index < lt {
            right = lt - 1;
        } else {
            left = gt + 1;
        }
    }
    // [left, lt-1] < pivotVal
    panic!("unreachable!!!")
}
