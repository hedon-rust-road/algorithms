use super::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![0; n];
        answer[0] = 1;

        for i in 1..n {
            answer[i] = answer[i - 1] * nums[i - 1];
        }

        let mut r = 1;
        for i in (0..=n - 1).rev() {
            answer[i] = answer[i] * r;
            r *= nums[i];
        }

        answer
    }
}
