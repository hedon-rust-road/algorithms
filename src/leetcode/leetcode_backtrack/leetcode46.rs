use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut tmp = vec![];
        let mut used = HashSet::with_capacity(nums.len());
        backtrack(&nums, &mut tmp, &mut used, &mut res);
        res
    }
}

fn backtrack(
    nums: &Vec<i32>,
    tmp: &mut Vec<i32>,
    used: &mut HashSet<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if tmp.len() == nums.len() {
        res.push(tmp.clone());
        return;
    }

    for num in nums {
        if used.contains(num) {
            continue;
        }
        tmp.push(*num);
        used.insert(*num);
        backtrack(nums, tmp, used, res);
        tmp.pop();
        used.remove(num);
    }
}
