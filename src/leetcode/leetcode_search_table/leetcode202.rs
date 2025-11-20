use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut record = HashSet::new();
    let mut n = n as i64;

    loop {
        n = cal_power(n);
        if n == 1 {
            return true;
        }
        println!("cal_power: {n}");
        if record.contains(&n) {
            return false;
        }
        record.insert(n);
    }
}

fn cal_power(mut n: i64) -> i64 {
    // 123
    let mut result = 0;
    while n != 0 {
        let num = n % 10;
        result += num * num;
        n = n / 10;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_happy_should_work() {
        assert!(is_happy(19));
    }
}
