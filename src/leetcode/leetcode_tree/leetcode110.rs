use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        height(&root) != -1
    }
}

fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            let left_height = height(&node.left);
            if left_height == -1 {
                return -1;
            }
            let right_height = height(&node.right);
            if right_height == -1 {
                return -1;
            }
            if (left_height - right_height).abs() > 1 {
                -1
            } else {
                1 + left_height.max(right_height)
            }
        }
        None => 0,
    }
}
