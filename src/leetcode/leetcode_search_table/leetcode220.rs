use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let value_diff = value_diff as i64;
        let bucket_size = value_diff + 1;
        let mut buckets = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let num = num as i64;
            let bucket_id = get_bucket_id(num, bucket_size);

            // 检测同一个桶
            if buckets.contains_key(&bucket_id) {
                return true;
            }

            // 检查相邻桶
            if let Some(val) = buckets.get(&(bucket_id - 1)) {
                if (num - val).abs() <= value_diff {
                    return true;
                }
            }
            if let Some(val) = buckets.get(&(bucket_id + 1)) {
                if (num - val).abs() <= value_diff {
                    return true;
                }
            }

            buckets.insert(bucket_id, num);

            if i >= index_diff as usize {
                let old_num = nums[i - index_diff as usize] as i64;
                buckets.remove(&get_bucket_id(old_num, bucket_size));
            }
        }

        false
    }

    // value_diff = 3
    // bucket_size = 4
    // [0, 1, 2, 3, 4]
    // num=1
    // bucketID = 1/4 = 0
}

fn get_bucket_id(num: i64, bucket_size: i64) -> i64 {
    if num >= 0 {
        num / bucket_size
    } else {
        (num + 1) / bucket_size - 1
    }
}
