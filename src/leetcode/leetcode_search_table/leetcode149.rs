use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }
        let mut result = 1;

        for (i, pi) in points.iter().enumerate() {
            let mut m = HashMap::new();
            for (j, pj) in points.iter().enumerate() {
                if i == j {
                    continue;
                }
                *m.entry(gradient(pi, pj)).or_insert(0) += 1;
            }
            for count in m.values() {
                if count + 1 > result {
                    result = *count + 1;
                }
            }
        }

        result
    }
}

fn gradient(pi: &Vec<i32>, pj: &Vec<i32>) -> String {
    let mut dx = pj[0] - pi[0];
    let mut dy = pj[1] - pi[1];
    let g = gcd(dx, dy);
    dx /= g;
    dy /= g;
    if dx < 0 {
        dx = -dx;
        dy = -dy;
    }
    format!("{dy}/{dx}")
}

fn gcd(dx: i32, dy: i32) -> i32 {
    let mut a = dx.abs();
    let mut b = dy.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
