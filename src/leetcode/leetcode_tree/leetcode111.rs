use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            let left = root.left.take();
            let right = root.right.take();
            if left.is_none() && right.is_none() {
                return 1;
            }
            if left.is_none() {
                return Self::min_depth(right) + 1;
            }
            if right.is_none() {
                return Self::min_depth(left) + 1;
            }
            let left = Self::min_depth(left) + 1;
            let right = Self::min_depth(right) + 1;
            if left < right { left } else { right }
        } else {
            0
        }
    }
}
