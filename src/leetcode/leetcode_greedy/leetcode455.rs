use super::Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut i = g.len() as i32 - 1;
        let mut j = s.len() as i32 - 1;

        let mut res = 0;
        while i >= 0 && j >= 0 {
            if s[j as usize] >= g[i as usize] {
                i -= 1;
                j -= 1;
                res += 1;
            } else {
                i -= 1;
            }
        }
        res
    }
}
