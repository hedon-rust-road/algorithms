use super::Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() || target <= 0 {
            return vec![];
        }
        candidates.sort();
        let mut res = vec![];
        let mut tmp = vec![];
        backtrack(&candidates, 0, target, &mut tmp, &mut res);
        res
    }
}

fn backtrack(
    candidates: &Vec<i32>,
    start: usize,
    target: i32,
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
        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }
        tmp.push(candidate);
        backtrack(candidates, i + 1, target - candidate, tmp, res);
        tmp.pop();
    }
}
