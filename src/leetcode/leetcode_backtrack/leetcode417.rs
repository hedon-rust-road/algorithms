use super::Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() || heights[0].is_empty() {
            return vec![];
        }

        let m = heights.len();
        let n = heights[0].len();
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];

        // 从边界开始 DFS
        for i in 0..m {
            dfs(&heights, &mut pacific, &directions, i, 0);
            dfs(&heights, &mut atlantic, &directions, i, n - 1);
        }

        for j in 0..n {
            dfs(&heights, &mut pacific, &directions, 0, j);
            dfs(&heights, &mut atlantic, &directions, m - 1, j);
        }

        // 收集结果
        let mut res = vec![];
        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }
}

fn dfs(
    heights: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    directions: &Vec<(i32, i32)>,
    i: usize,
    j: usize,
) {
    if visited[i][j] {
        return;
    }
    visited[i][j] = true;

    for direction in directions {
        let new_i = i as i32 + direction.0;
        let new_j = j as i32 + direction.1;

        if new_i < 0 || new_j < 0 {
            continue;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;

        if new_i < heights.len()
            && new_j < heights[0].len()
            && heights[new_i][new_j] >= heights[i][j]
        {
            dfs(heights, visited, directions, new_i, new_j);
        }
    }
}
