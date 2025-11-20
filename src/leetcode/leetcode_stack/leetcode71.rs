use super::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        let items: Vec<&str> = path.split("/").collect();

        for item in items {
            match item {
                "" | "." => continue,
                ".." => _ = stack.pop(),
                _ => stack.push(item),
            }
        }

        "/".to_string() + &stack.join("/")
    }
}
