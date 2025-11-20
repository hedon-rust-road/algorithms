use super::{Solution, TreeNode};

struct LevelTreeNode {
    level: usize,
    node: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(LevelTreeNode {
                level: 1,
                node: root,
            });
        }

        while let Some(node) = queue.pop_front() {
            let level = node.level;
            if res.len() < level {
                res.push(vec![]);
            }

            if let Some(node) = node.node {
                let node = node.borrow();
                if level % 2 == 1 {
                    res[level - 1].push(node.val);
                } else {
                    res[level - 1].insert(0, node.val);
                }

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

        res
    }
}
