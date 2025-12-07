use super::Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return 0;
        }

        intervals.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let n = intervals.len();
        let mut res = 1;
        let mut pre = 0;

        for i in 1..n {
            if intervals[i][0] >= intervals[pre][1] {
                pre = i;
                res += 1;
            }
        }

        n as i32 - res
    }
}
