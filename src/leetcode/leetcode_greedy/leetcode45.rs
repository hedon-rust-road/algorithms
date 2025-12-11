use super::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut steps = 0;
        let mut end = 0;
        let mut max_pos = 0;

        for i in 0..nums.len() - 1 {
            if i + nums[i] as usize > max_pos {
                max_pos = i + nums[i] as usize;
            }

            if i == end {
                steps += 1;
                end = max_pos;
                if max_pos >= nums.len() - 1 {
                    break;
                }
            }
        }

        steps
    }
}
