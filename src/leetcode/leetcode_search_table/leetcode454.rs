use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut two_sum = HashMap::new();
        for &num1 in &nums1 {
            for &num2 in &nums2 {
                *two_sum.entry(num1 + num2).or_insert(0) += 1
            }
        }

        nums3
            .iter()
            .flat_map(|&num3| nums4.iter().map(move |num4| num3 + num4))
            .filter_map(|sum| two_sum.get(&-sum))
            .sum()
    }
}
