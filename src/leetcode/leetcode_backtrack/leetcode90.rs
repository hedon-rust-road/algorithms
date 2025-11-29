use super::Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        let mut tmp = vec![];
        nums.sort();
        backtrack(&nums, 0, &mut tmp, &mut res);
        return res;
    }
}

fn backtrack(nums: &Vec<i32>, start: usize, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }
        tmp.push(nums[i]);
        res.push(tmp.clone());
        backtrack(nums, i + 1, tmp, res);
        tmp.pop();
    }
}
