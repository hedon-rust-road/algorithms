use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        match root {
            None => vec![],
            Some(node) => {
                let node_ref = node.borrow();
                let val = node_ref.val.to_string();

                // 叶子节点
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return vec![val];
                }

                // 收集左右子树的路径
                let mut paths = vec![];
                for child in [&node_ref.left, &node_ref.right].iter() {
                    if let Some(child_node) = child {
                        for path in Self::binary_tree_paths(Some(child_node.clone())) {
                            paths.push(format!("{}->{}", val, path));
                        }
                    }
                }
                paths
            }
        }
    }
}
