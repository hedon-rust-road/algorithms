use std::collections::VecDeque;

use super::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut queue = VecDeque::new();
        // (num, 到 n 走了多少步)
        queue.push_back((n, 0));

        let mut visited: Vec<bool> = vec![false; n as usize + 1];
        while let Some((num, step)) = queue.pop_front() {
            if num == 0 {
                return step;
            }

            for i in 0.. {
                // 经过一步
                let a = num - i * i;
                if a < 0 {
                    break;
                }
                if a == 0 {
                    return step + 1;
                }

                // 没有访问过
                if !visited[a as usize] {
                    queue.push_back((a, step + 1));
                    visited[a as usize] = true;
                }
            }
        }

        panic!("unreachable!")
    }
}
