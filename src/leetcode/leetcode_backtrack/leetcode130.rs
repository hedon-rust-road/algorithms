use super::Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        let mut visited = vec![vec![false; n]; m];
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

        // 上下
        for j in 0..n {
            dsf(board, &mut visited, &directions, 0, j as i32);
            if m > 1 {
                dsf(board, &mut visited, &directions, m as i32 - 1, j as i32);
            }
        }

        // 左右
        for i in 0..m {
            dsf(board, &mut visited, &directions, i as i32, 0);
            if n > 1 {
                dsf(board, &mut visited, &directions, i as i32, n as i32 - 1);
            }
        }

        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] {
                    board[i][j] = 'X'
                }
            }
        }
    }
}

fn dsf(
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    directions: &Vec<(i32, i32)>,
    i: i32,
    j: i32,
) {
    if board[i as usize][j as usize] != 'O' || visited[i as usize][j as usize] {
        return;
    }
    visited[i as usize][j as usize] = true;

    for direction in directions {
        let new_i = i + direction.0;
        let new_j = j + direction.1;
        if new_i < 0 || new_j < 0 {
            continue;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if new_i < board.len()
            && new_j < board[new_i].len()
            && !visited[new_i][new_j]
            && board[new_i][new_j] == 'O'
        {
            dsf(board, visited, directions, new_i as i32, new_j as i32);
        }
    }
}
