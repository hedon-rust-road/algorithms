use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            is_same(left, right)
        } else {
            false
        }
    }
}

fn is_same(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (node1, node2) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(node1), Some(node2)) => {
            if node1.borrow().val != node2.borrow().val {
                false
            } else {
                is_same(
                    node1.borrow_mut().left.take(),
                    node2.borrow_mut().right.take(),
                ) && is_same(
                    node1.borrow_mut().right.take(),
                    node2.borrow_mut().left.take(),
                )
            }
        }
    }
}
