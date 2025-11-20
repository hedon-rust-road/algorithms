use std::i64;

use super::Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut result = i64::MAX;
        let mut diff = i64::MAX;
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] as i64 + nums[left] as i64 + nums[right] as i64;
                let abs = (sum - target as i64).abs();
                if abs == 0 {
                    return target;
                }
                // 更新最小值
                if abs < diff {
                    result = sum;
                    diff = abs;
                }

                // 继续尝试
                if sum > target as i64 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }

        result as i32
    }
}
