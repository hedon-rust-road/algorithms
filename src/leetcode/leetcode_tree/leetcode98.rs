use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        validate(&root, None, None)
    }
}

fn validate(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    match node {
        None => true,
        Some(node) => {
            let val = node.borrow().val;

            if let Some(min_val) = min {
                if val <= min_val {
                    return false;
                }
            }

            if let Some(max_val) = max {
                if val >= max_val {
                    return false;
                }
            }

            let node_def = node.borrow();
            validate(&node_def.left, min, Some(val)) && validate(&node_def.right, Some(val), max)
        }
    }
}
