use super::Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 2 {
            return 1;
        }
        let mut mem = vec![0; n as usize + 1];
        mem[1] = 1;
        mem[2] = 1;
        dp(n as usize, &mut mem) as i32
    }
}

fn dp(n: usize, mem: &mut Vec<usize>) -> usize {
    if mem[n] != 0 {
        return mem[n];
    }
    let mut max = 0;
    for i in 1..=n - 1 {
        max = max.max(i * (n - i)).max((n - i) * dp(i, mem));
    }
    mem[n] = max;
    max
}

fn max3(a: usize, b: usize, c: usize) -> usize {
    a.max(b).max(c)
}
