use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1_map = HashMap::new();
    for num1 in nums1 {
        *nums1_map.entry(num1).or_insert(0) += 1;
    }

    nums2.into_iter().fold(Vec::new(), |mut acc, x| {
        if let Some(count) = nums1_map.get_mut(&x) {
            if *count > 0 {
                *count -= 1;
                acc.push(x);
            }
        }
        acc
    })
}
