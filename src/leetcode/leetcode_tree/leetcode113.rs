use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        match root {
            Some(root) => {
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                let val = root.borrow().val;
                if left.is_none() && right.is_none() && target_sum == val {
                    return vec![vec![val]];
                }
                let mut res = vec![];
                for node in vec![left, right] {
                    if node.is_some() {
                        for path in Self::path_sum(node, target_sum - val) {
                            let mut tmp = vec![val];
                            tmp.extend(path);
                            res.push(tmp);
                        }
                    }
                }
                res
            }
            None => vec![],
        }
    }
}
