use super::{Solution, TreeNode};

struct LevelTreeNode {
    level: usize,
    node: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(LevelTreeNode {
            level: 1,
            node: root,
        });
        while let Some(node) = queue.pop_front() {
            let level = node.level;
            if let Some(node) = &node.node {
                if res.len() == level - 1 {
                    res.push(vec![]);
                }
                if let Ok(node) = node.try_borrow() {
                    res[level - 1].push(node.val);
                    if node.left.is_some() {
                        queue.push_back(LevelTreeNode {
                            level: level + 1,
                            node: node.left.clone(),
                        });
                    }
                    if node.right.is_some() {
                        queue.push_back(LevelTreeNode {
                            level: level + 1,
                            node: node.right.clone(),
                        });
                    }
                }
            }
        }
        res
    }
}
