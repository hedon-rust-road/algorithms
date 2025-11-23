use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let node_val = node.borrow().val;
                match node_val.cmp(&key) {
                    std::cmp::Ordering::Less => {
                        let right = node.borrow_mut().right.take();
                        node.borrow_mut().right = Self::delete_node(right, key);
                    }
                    std::cmp::Ordering::Greater => {
                        let left = node.borrow_mut().left.take();
                        node.borrow_mut().left = Self::delete_node(left, key);
                    }
                    std::cmp::Ordering::Equal => {
                        // 删除当前节点，需要提升子树
                        let left = node.borrow_mut().left.take();
                        let right = node.borrow_mut().right.take();
                        match (left, right) {
                            (None, None) => return None,
                            (None, Some(r)) => return Some(r),
                            (Some(l), None) => return Some(l),
                            (Some(l), Some(r)) => {
                                let mut current = r.clone();
                                while current.borrow().left.is_some() {
                                    let next = current.borrow().left.clone().unwrap();
                                    current = next;
                                }
                                current.borrow_mut().left = Some(l);
                                return Some(r);
                            }
                        }
                    }
                }
                Some(node)
            }
        }
    }
}
