use rand::Rng;

use super::Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        quicksort(&mut nums, 0, n - 1);
        return nums;
    }
}

fn quicksort(nums: &mut Vec<i32>, start: i32, end: i32) {
    if start >= end {
        return;
    }
    let mut l = start as usize;
    let mut r = end as usize;

    // 随机选择 pivot
    let rand_index = rand::rng().random_range(l..=r);
    nums.swap(l, rand_index);
    let pivot = nums[l];

    while l < r {
        // 从右往左找到第一个小于 pivot 的值
        while l < r && nums[r] >= pivot {
            r -= 1;
        }

        // 直接填入左边的坑（也就是 l 的位置）
        // 此时 r 变成了新的坑
        nums.swap(l, r);

        // 从左往右找到第一个大于 pivot 的值
        while l < r && nums[l] < pivot {
            l += 1;
        }
        nums.swap(l, r);
    }

    nums[l] = pivot;

    quicksort(nums, start, l as i32 - 1);
    quicksort(nums, l as i32 + 1, end);
}
