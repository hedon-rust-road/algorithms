use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    false
                } else {
                    Self::is_same_tree(p.borrow_mut().left.take(), q.borrow_mut().left.take())
                        && Self::is_same_tree(
                            p.borrow_mut().right.take(),
                            q.borrow_mut().right.take(),
                        )
                }
            }
        }
    }
}
