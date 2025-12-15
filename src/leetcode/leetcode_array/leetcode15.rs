use super::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut res = vec![];

        for (i, num) in nums.iter().enumerate() {
            if num > &0 {
                break;
            }
            if i > 0 && num == &nums[i - 1] {
                continue;
            }

            // 转换为两数之和
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            let target = -num;

            while left < right {
                let sum = nums[left] + nums[right];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        res.push(vec![*num, nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1
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

        res
    }
}
