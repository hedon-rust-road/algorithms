use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let paths = build_paths(root);
        let numbers = paths_2_numbers(paths);
        numbers.iter().sum()
    }
}

fn build_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    match root {
        Some(root) => {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            let val = root.borrow().val;

            if left.is_none() && right.is_none() {
                vec![vec![val]]
            } else {
                let mut res = vec![];
                for node in vec![left, right] {
                    if node.is_some() {
                        for mut path in build_paths(node) {
                            path.push(val);
                            res.push(path);
                        }
                    }
                }
                res
            }
        }
        None => vec![],
    }
}

fn paths_2_numbers(paths: Vec<Vec<i32>>) -> Vec<i32> {
    paths.into_iter().map(|path| path_2_number(path)).collect()
}

fn path_2_number(path: Vec<i32>) -> i32 {
    let mut res = 0;
    for (i, mut item) in path.into_iter().enumerate() {
        for _ in 0..i {
            item *= 10;
        }
        res += item
    }
    res
}
