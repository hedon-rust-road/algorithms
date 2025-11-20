use super::Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let target = target as i64;
        let mut result = vec![];
        if nums.len() <= 3 {
            return result;
        }

        // 固定第 1 位
        for i in 0..nums.len() - 3 {
            // 如果是重复的，则再尝试也是得到一样的结果，直接跳过
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            // 固定第 2 位
            for j in i + 1..nums.len() - 2 {
                // 如果是重复的，则再尝试也是得到一样的结果，直接跳过
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let want = target - nums[i] as i64 - nums[j] as i64;

                // 剩下的就是两数之和了，在 [j+1, len-1] 之间找 3/4 位
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum = nums[left] as i64 + nums[right] as i64;
                    match sum.cmp(&want) {
                        std::cmp::Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            left += 1;
                            right -= 1;
                            while left < right && nums[left] == nums[left - 1] {
                                left += 1
                            }
                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1
                            }
                        }
                        std::cmp::Ordering::Less => left += 1,
                        std::cmp::Ordering::Greater => right -= 1,
                    }
                }
            }
        }

        result
    }
}
