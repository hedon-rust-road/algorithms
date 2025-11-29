use super::Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if k <= 0 || k > 9 || n <= 0 || n > 81 {
            return vec![];
        }
        let mut res = vec![];
        let mut tmp = vec![];
        backtrack(k, n, 1, &mut tmp, &mut res);
        res
    }
}

fn backtrack(k: i32, n: i32, start: usize, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if tmp.len() == k as usize {
        if n == 0 {
            res.push(tmp.clone());
        }
        return;
    }

    for i in start..=9 {
        if n - (i as i32) < 0 {
            break;
        }
        tmp.push(i as i32);
        backtrack(k, n - i as i32, i + 1, tmp, res);
        tmp.pop();
    }
}
