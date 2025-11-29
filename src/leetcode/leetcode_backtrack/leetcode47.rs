use super::Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        nums.sort();
        let mut res = vec![];
        let mut tmp = vec![];
        let mut used = vec![false; nums.len()]; // key: index
        backtrack(&nums, &mut tmp, &mut used, &mut res);
        res
    }
}

fn backtrack(nums: &Vec<i32>, tmp: &mut Vec<i32>, used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
    if tmp.len() == nums.len() {
        res.push(tmp.clone());
        return;
    }

    for (i, num) in nums.iter().enumerate() {
        if used[i] {
            continue;
        }

        // 如果当前数字和前一个相同，且前一个还没用过，就跳过
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
            continue;
        }

        tmp.push(*num);
        used[i] = true;

        backtrack(nums, tmp, used, res);

        tmp.pop();
        used[i] = false;
    }
}
