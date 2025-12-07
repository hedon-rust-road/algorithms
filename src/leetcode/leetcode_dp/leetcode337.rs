use super::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (rob, not_rob) = rob3_dp(&root);
        rob.max(not_rob)
    }
}

fn rob3_dp(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }

    let root = root.as_ref().unwrap().borrow();

    let left = rob3_dp(&root.left);
    let right = rob3_dp(&root.right);

    // 偷 root，则左右子树都不可偷
    let rob = root.val + left.1 + right.1;

    // 不偷 root，则左右子树可偷可不偷
    let not_rob = left.0.max(left.1) + right.0.max(right.1);

    (rob, not_rob)
}
