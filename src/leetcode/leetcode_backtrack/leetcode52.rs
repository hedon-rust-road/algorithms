use super::Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let n = n as usize;
        let mut cols = vec![false; n];
        let mut dia1 = vec![false; 2 * n - 1];
        let mut dia2 = vec![false; 2 * n - 1];

        let mut res = 0;
        backtrack(n, 0, &mut res, &mut cols, &mut dia1, &mut dia2);
        res
    }
}

fn backtrack(
    n: usize,
    index: usize,
    res: &mut i32,
    cols: &mut Vec<bool>,
    dia1: &mut Vec<bool>,
    dia2: &mut Vec<bool>,
) {
    if index == n {
        *res += 1;
        return;
    }

    for i in 0..n {
        if !cols[i] && !dia1[index + i] && !dia2[index - i + n - 1] {
            cols[i] = true;
            dia1[index + i] = true;
            dia2[index - i + n - 1] = true;

            backtrack(n, index + 1, res, cols, dia1, dia2);
            cols[i] = false;
            dia1[index + i] = false;
            dia2[index - i + n - 1] = false;
        }
    }
}
