use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut record = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let want = target - num;
            if let Some(before) = record.get(&want) {
                return vec![*before as i32, idx as i32];
            }
            record.insert(num, idx);
        }
        vec![]
    }
}
