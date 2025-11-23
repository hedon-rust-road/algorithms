use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum_3(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        match root {
            Some(root) => {
                // 包含 root 的情况
                let mut res = find_path(Some(root.clone()), target_sum as i64);
                // 不包含 root 的情况
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                res += Self::path_sum_3(left, target_sum);
                res += Self::path_sum_3(right, target_sum);
                res
            }
            None => 0,
        }
    }
}

// 包含 node 节点的情况
fn find_path(node: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
    match node {
        Some(node) => {
            let left = node.borrow_mut().left.clone();
            let right = node.borrow_mut().right.clone();
            let val = node.borrow().val as i64;
            let mut res = 0;
            if val == target_sum {
                res += 1;
            }
            res += find_path(left, target_sum - val);
            res += find_path(right, target_sum - val);
            res
        }
        None => 0,
    }
}
