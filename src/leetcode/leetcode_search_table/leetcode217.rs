use super::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut record = HashSet::new();
        for num in nums {
            if record.get(&num).is_some() {
                return true;
            }
            record.insert(num);
        }
        return false;
    }
}
