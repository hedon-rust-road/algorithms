use super::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            n => {
                let mut a = 1;
                let mut b = 2;
                for _ in 3..=n {
                    let c = a + b;
                    a = b;
                    b = c;
                }
                b
            }
        }
    }
}
