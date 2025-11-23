use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor_in_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_val = root.as_ref().unwrap().borrow().val;
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        if p_val < root_val && q_val < root_val {
            Self::lowest_common_ancestor_in_bst(
                root.as_ref().unwrap().borrow_mut().left.take(),
                p,
                q,
            )
        } else if p_val > root_val && q_val > root_val {
            Self::lowest_common_ancestor_in_bst(
                root.as_ref().unwrap().borrow_mut().right.take(),
                p,
                q,
            )
        } else {
            root
        }
    }
}
