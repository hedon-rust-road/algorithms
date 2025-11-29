use super::Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let n = board.len();

        let mut rows = vec![vec![false; n]; n + 1]; // rows[num][i] 表示 num 第 i 列存在
        let mut cols = vec![vec![false; n]; n + 1]; // cols[num][j] 表示 num 第 j 列存在
        let mut boxes = vec![vec![false; n]; n + 1]; // boxes[num][box_index] 表示 num 第 box_index 个格子中存在

        // 初始化一下 cols, rows, boxes
        for i in 0..n {
            for j in 0..n {
                if board[i][j] != '.' {
                    let num = board[i][j] as usize - b'0' as usize;
                    rows[num][i] = true;
                    cols[num][j] = true;
                    boxes[num][box_index(i, j)] = true;
                }
            }
        }

        backtrack(board, &mut rows, &mut cols, &mut boxes, 0, 0);
    }
}

fn backtrack(
    board: &mut Vec<Vec<char>>,
    rows: &mut Vec<Vec<bool>>,
    cols: &mut Vec<Vec<bool>>,
    boxes: &mut Vec<Vec<bool>>,
    start_i: usize,
    start_j: usize,
) -> bool {
    let n = board.len();

    for i in start_i..n {
        let mut start = 0;
        if i == start_i {
            start = start_j;
        }
        for j in start..n {
            // 找到第一个非数字的
            if board[i][j] == '.' {
                // 尝试填充 1-9，如果都失败，则返回 false
                for k in 1..=9 {
                    if !rows[k][i] && !cols[k][j] && !boxes[k][box_index(i, j)] {
                        board[i][j] = (k as u8 + b'0') as char;
                        rows[k][i] = true;
                        cols[k][j] = true;
                        boxes[k][box_index(i, j)] = true;

                        if j < n - 1 {
                            // 往右走
                            if backtrack(board, rows, cols, boxes, i, j + 1) {
                                return true;
                            }
                        } else {
                            // 往下走
                            if backtrack(board, rows, cols, boxes, i + 1, 0) {
                                return true;
                            }
                        }

                        board[i][j] = '.';
                        rows[k][i] = false;
                        cols[k][j] = false;
                        boxes[k][box_index(i, j)] = false;
                    }
                }
                return false;
            }
        }
    }

    true
}

fn box_index(i: usize, j: usize) -> usize {
    return i / 3 * 3 + j / 3;
}
