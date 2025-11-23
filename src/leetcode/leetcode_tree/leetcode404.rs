use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let mut result = 0;
            if let Some(left) = left {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    result += left.borrow().val;
                } else {
                    result += Self::sum_of_left_leaves(Some(left));
                }
            }
            result + Self::sum_of_left_leaves(right)
        } else {
            0
        }
    }
}
