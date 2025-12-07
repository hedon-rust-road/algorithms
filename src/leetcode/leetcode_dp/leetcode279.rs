use std::i32;

use super::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut mem = vec![i32::MAX; n + 1];
        mem[0] = 0;
        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                mem[i] = mem[i].min(mem[i - j * j] + 1);
                j += 1;
            }
        }
        mem[n]
    }
}
