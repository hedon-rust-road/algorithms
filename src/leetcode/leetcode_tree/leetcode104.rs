use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            let left = Self::max_depth(root.left.take()) + 1;
            let right = Self::max_depth(root.right.take()) + 1;
            if left > right { left } else { right }
        } else {
            0
        }
    }
}
