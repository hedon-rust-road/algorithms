use super::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 0;
        }
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }
        let mut dp = vec![vec![0; n]; m];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i == m - 1 && j == n - 1 {
                    dp[i][j] = 1;
                    continue;
                }
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
                // 往右
                if i + 1 < m && obstacle_grid[i + 1][j] != 1 {
                    dp[i][j] += dp[i + 1][j]
                }
                // 往下
                if j + 1 < n && obstacle_grid[i][j + 1] != 1 {
                    dp[i][j] += dp[i][j + 1]
                }
            }
        }
        dp[0][0]
    }
}
