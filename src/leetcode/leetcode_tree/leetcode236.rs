use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor_in_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().map(|node| node.borrow().val);
        let q_val = q.as_ref().map(|node| node.borrow().val);
        helper(&root, p_val, q_val)
    }
}

fn helper(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p_val: Option<i32>,
    q_val: Option<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            let val = node.borrow().val;
            if Some(val) == p_val || Some(val) == q_val {
                return Some(node.clone());
            }

            let node_ref = node.borrow();
            let left = helper(&node_ref.left, p_val, q_val);
            let right = helper(&node_ref.right, p_val, q_val);

            match (left, right) {
                (Some(_), Some(_)) => Some(node.clone()),
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                _ => None,
            }
        }
    }
}
