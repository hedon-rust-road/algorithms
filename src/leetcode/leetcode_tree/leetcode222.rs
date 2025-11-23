use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            match (left, right) {
                (None, None) => 1,
                (None, Some(_)) => 0,
                (left, right) => Self::count_nodes(left) + Self::count_nodes(right) + 1,
            }
        } else {
            0
        }
    }
}
