use super::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() <= 1 {
            return;
        }
        let n = nums.len();
        let k = k % (n as i32);
        reverse(nums, 0, n as i32 - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n as i32 - 1);
    }
}

fn reverse(nums: &mut Vec<i32>, start: i32, end: i32) {
    if start < 0 || end < 0 {
        return;
    }
    let mut start = start as usize;
    let mut end = end as usize;
    while start < end {
        nums.swap(start, end);
        start += 1;
        end -= 1;
    }
}
