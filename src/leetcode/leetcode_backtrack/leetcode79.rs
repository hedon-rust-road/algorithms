use super::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || word.is_empty() {
            return false;
        }

        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let word_bytes = word.as_bytes();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == word_bytes[0] as char {
                    visited[i][j] = true;
                    if backtrack(
                        &board,
                        i as i32,
                        j as i32,
                        1,
                        word_bytes,
                        &directions,
                        &mut visited,
                    ) {
                        return true;
                    }
                    visited[i][j] = false;
                }
            }
        }
        false
    }
}

fn backtrack(
    board: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    index: usize,
    word: &[u8],
    directions: &Vec<(i32, i32)>,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if index == word.len() {
        return true;
    }

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
            && board[new_i][new_j] == word[index] as char
        {
            visited[new_i][new_j] = true;
            if backtrack(
                board,
                new_i as i32,
                new_j as i32,
                index + 1,
                word,
                directions,
                visited,
            ) {
                return true;
            }
            visited[new_i][new_j] = false;
        }
    }

    false
}
