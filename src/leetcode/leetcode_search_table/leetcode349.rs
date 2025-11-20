use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let num1_set: HashSet<i32> = nums1.into_iter().collect();
    let num2_set: HashSet<i32> = nums2.into_iter().collect();

    num1_set.intersection(&num2_set).copied().collect()
}
