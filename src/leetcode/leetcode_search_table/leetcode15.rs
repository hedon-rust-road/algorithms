use super::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut result = vec![];
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                // 如果最小值都大于 0，不可能有解
                break;
            }
            // 尝试 nums[i] 和 nums[i+1..]
            if i > 0 && nums[i] == nums[i - 1] {
                // 跟前面一样的话，如果尝试成功了，那也是重复的，所以不需要尝试
                continue;
            }

            // 转化为求两数之和
            let target = 0 - nums[i];
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[left] + nums[right];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        // 去重，重复的元素不需要再次尝试了
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                }
            }
        }

        result
    }
}
