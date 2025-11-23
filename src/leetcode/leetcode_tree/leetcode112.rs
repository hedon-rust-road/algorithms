use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let val = node.borrow().val;
            if left.is_none() && right.is_none() && val == target_sum {
                true
            } else {
                Self::has_path_sum(left, target_sum - val)
                    || Self::has_path_sum(right, target_sum - val)
            }
        } else {
            false
        }
    }
}
