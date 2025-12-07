use super::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return n as i32;
        }
        let mut dp: Vec<i32> = vec![0; n];
        dp[0] = 1;

        for i in 1..n {
            let mut lis = 0;
            for j in (0..=i - 1).rev() {
                if nums[j] < nums[i] {
                    if dp[j] > lis {
                        lis = dp[j]
                    }
                }
            }
            dp[i] = lis + 1;
        }

        *dp.iter().max().unwrap()
    }
}
