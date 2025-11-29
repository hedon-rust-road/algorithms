use super::Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n <= 0 {
            return vec![];
        }
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut dia1 = vec![false; 2 * n - 1];
        let mut dia2 = vec![false; 2 * n - 1];

        let mut res = vec![];
        let mut tmp = vec![];
        backtrack(n, 0, &mut tmp, &mut res, &mut cols, &mut dia1, &mut dia2);
        res
    }
}

fn backtrack(
    n: usize,
    index: usize,
    tmp: &mut Vec<usize>,
    res: &mut Vec<Vec<String>>,
    cols: &mut Vec<bool>,
    dia1: &mut Vec<bool>,
    dia2: &mut Vec<bool>,
) {
    if index == n {
        res.push(generate_queue(tmp));
        return;
    }

    for i in 0..n {
        if !cols[i] && !dia1[index + i] && !dia2[index - i + n - 1] {
            tmp.push(i);
            cols[i] = true;
            dia1[index + i] = true;
            dia2[index - i + n - 1] = true;

            backtrack(n, index + 1, tmp, res, cols, dia1, dia2);

            tmp.pop();
            cols[i] = false;
            dia1[index + i] = false;
            dia2[index - i + n - 1] = false;
        }
    }
}

fn generate_queue(indexes: &Vec<usize>) -> Vec<String> {
    let n = indexes.len();
    let mut res = Vec::with_capacity(n as usize);
    for index in indexes {
        let mut tmp = "".to_string();
        for i in 0..n {
            if i == *index {
                tmp.push('Q');
            } else {
                tmp.push('.');
            }
        }
        res.push(tmp);
    }
    res
}
