use super::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut record = HashSet::new();
        for (i, num) in nums.iter().enumerate() {
            if record.get(num).is_some() {
                return true;
            }
            record.insert(num);
            if i >= k as usize {
                record.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}
