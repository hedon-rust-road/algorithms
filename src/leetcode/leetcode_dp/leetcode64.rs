use std::{
    cmp::min,
    i32::{self},
};

use super::Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if i == n - 1 && j == m - 1 {
                    continue;
                }

                let right = if j + 1 < m { grid[i][j + 1] } else { i32::MAX };
                let bottom = if i + 1 < n { grid[i + 1][j] } else { i32::MAX };
                grid[i][j] += min(right, bottom);
            }
        }
        grid[0][0]
    }
}
