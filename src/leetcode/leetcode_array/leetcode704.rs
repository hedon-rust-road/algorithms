// https://leetcode.cn/problems/binary-search/description/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }

    // 在 [l, r] 中进行查找
    let mut l = 0 as i32;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let mid = (r - l) / 2 + l;
        let mid_val = nums[mid as usize];
        if mid_val == target {
            return mid as i32;
        } else if mid_val > target {
            // 在左边继续找
            r = mid - 1
        } else {
            // 在右边继续找
            l = mid + 1
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_should_work() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(search(arr.clone(), 0), -1);
        assert_eq!(search(arr.clone(), 1), 0);
        assert_eq!(search(arr.clone(), 5), 2);
        assert_eq!(search(arr.clone(), 9), 4);
        assert_eq!(search(arr.clone(), 10), -1);
    }
}
