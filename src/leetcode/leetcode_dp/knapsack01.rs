use super::Solution;

impl Solution {
    fn knapsack01(weights: Vec<i32>, values: Vec<i32>, c: usize) -> i32 {
        if c <= 0 || weights.is_empty() || values.is_empty() {
            return 0;
        }

        let n = weights.len();
        let mut dp = vec![0; c + 1];

        for i in 0..n {
            let w = weights[i] as usize;
            let v = values[i];
            for j in (w..=c).rev() {
                dp[j] = dp[j].max(v + dp[j - w]);
            }
        }
        dp[c]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knapsack01_should_work() {
        let res = Solution::knapsack01(vec![1, 2, 3], vec![6, 10, 12], 5);
        assert_eq!(res, 22);
    }
}
