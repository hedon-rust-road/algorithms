use super::Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let n = citations.len();
        let mut res = 0;
        for (i, citation) in citations.iter().enumerate().rev() {
            if (*citation as usize) < n - i {
                break;
            }
            res = n - i;
        }
        res as i32
    }
}
