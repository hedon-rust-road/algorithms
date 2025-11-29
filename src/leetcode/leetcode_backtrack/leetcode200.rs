use super::Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let mut res = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if !visited[i][j] && grid[i][j] == '1' {
                    visited[i][j] = true;
                    res += 1;
                    backtrack(&grid, &mut visited, &directions, i as i32, j as i32)
                }
            }
        }

        res
    }
}

fn backtrack(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    directions: &Vec<(i32, i32)>,
    i: i32,
    j: i32,
) {
    for direction in directions {
        let new_i = i + direction.0;
        let new_j = j + direction.1;
        if new_i < 0 || new_j < 0 {
            continue;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if new_i < grid.len()
            && new_j < grid[new_i].len()
            && !visited[new_i][new_j]
            && grid[new_i][new_j] == '1'
        {
            visited[new_i][new_j] = true;
            backtrack(grid, visited, directions, new_i as i32, new_j as i32);
        }
    }
}
