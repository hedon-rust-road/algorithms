use super::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.is_empty() || num_rows == 1 {
            return s;
        }

        let mut res: Vec<String> = vec![String::new(); num_rows as usize];
        let mut row = 0;
        let mut up = false;
        for ch in s.chars() {
            res[row].push(ch);
            (row, up) = next_position(row, num_rows, up);
        }

        res.concat()
    }
}

fn next_position(row: usize, num_rows: i32, up: bool) -> (usize, bool) {
    if row == 0 {
        return (row + 1, false);
    }
    if row == num_rows as usize - 1 {
        return (row - 1, true);
    }
    match up {
        true => (row - 1, true),
        false => (row + 1, false),
    }
}
