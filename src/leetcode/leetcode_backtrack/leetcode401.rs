use super::Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on == 0 {
            return vec!["0:00".to_string()];
        }
        if turned_on >= 9 {
            return vec![];
        }
        let values = vec![1, 2, 4, 8, 1, 2, 4, 8, 16, 32];
        let mut res = vec![];
        backtrack(turned_on, 0, 0, 0, &values, &mut res);
        res
    }
}

fn backtrack(
    turned_on: i32,
    start: usize,
    hour: i32,
    min: i32,
    values: &Vec<i32>,
    res: &mut Vec<String>,
) {
    if turned_on == 0 {
        if min < 10 {
            res.push(format!("{hour}:0{min}"));
        } else {
            res.push(format!("{hour}:{min}"));
        }
        return;
    }

    for i in start..values.len() {
        let value = values[i];
        if i <= 3 {
            // hour
            if hour + value < 12 {
                backtrack(turned_on - 1, i + 1, hour + value, min, values, res);
            }
        } else {
            // min
            if min + value < 60 {
                backtrack(turned_on - 1, i + 1, hour, min + value, values, res);
            }
        }
    }
}
