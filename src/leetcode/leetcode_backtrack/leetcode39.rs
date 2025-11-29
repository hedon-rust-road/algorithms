use super::Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() || target <= 0 {
            return vec![];
        }
        candidates.sort();
        let mut res = vec![];
        let mut tmp = vec![];
        backtrack(&candidates, target, 0, &mut tmp, &mut res);
        res
    }
}

fn backtrack(
    candidates: &Vec<i32>,
    target: i32,
    start: usize,
    tmp: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        res.push(tmp.clone());
        return;
    }

    for i in start..candidates.len() {
        let candidate = candidates[i];
        if target - candidate < 0 {
            break;
        }
        tmp.push(candidate);
        backtrack(candidates, target - candidate, i, tmp, res);
        tmp.pop();
    }
}
