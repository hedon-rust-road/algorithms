use super::Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.is_empty() {
            return 0;
        }
        let n = ratings.len();

        // 每一个默认至少有 1 个
        let mut candies = vec![1; n];

        // 只看左边
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        // 只看右边
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                // 保留最大值，这样可以同时满足左右两种情况
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}
