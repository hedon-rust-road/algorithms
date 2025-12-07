use super::Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }

        let target = (sum / 2) as usize;

        // dp[j] 表示是否可以凑出和为 j 的子数组
        let mut dp = vec![false; target + 1];
        dp[0] = true; // 容量为 0 时，总能达到（不选择任何元素）

        for num in nums {
            let num = num as usize;
            for j in (num..=target).rev() {
                // case1: dp[j] 已经可以了，直接返回
                // case2: dp[j-num] + num 可以，说明也凑到了
                dp[j] = dp[j] || dp[j - num];
                if dp[target] {
                    return true;
                }
            }
        }

        dp[target]
    }
}
