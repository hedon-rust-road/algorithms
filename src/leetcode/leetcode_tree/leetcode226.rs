use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();

            root.borrow_mut().left = Self::invert_tree(right);
            root.borrow_mut().right = Self::invert_tree(left);
            Some(root)
        } else {
            None
        }
    }
}
