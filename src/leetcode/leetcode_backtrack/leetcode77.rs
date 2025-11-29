use super::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut tmp = vec![];
        backtrack(n as usize, 1, k, &mut tmp, &mut res);
        res
    }
}

fn backtrack(n: usize, start: usize, k: i32, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if tmp.len() == k as usize {
        res.push(tmp.clone());
        return;
    }

    for i in start..=n {
        tmp.push(i as i32);
        backtrack(n, i + 1, k, tmp, res);
        tmp.pop();
    }
}
