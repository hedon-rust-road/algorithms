use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        helper(&root, k).0
    }
}

// 第一个返回值：结果，如果为 -1，表示没有找到
// 第二个返回值，没找到的时候返回 root 拥有的元素个数，用来更新 k
fn helper(root: &Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> (i32, i32) {
    match root {
        None => (-1, 0),
        Some(node) => {
            // 先在左边找
            let (left_result, left_count) = helper(&node.borrow().left, k);
            if left_result != -1 {
                return (left_result, 0);
            }

            // 左边没找到
            k = k - left_count;

            // 剩下一个，那就是当前 node
            if k == 1 {
                return (node.borrow().val, 0);
            }

            // 再找右边的
            k -= 1;
            let (right_result, right_count) = helper(&node.borrow().right, k);
            if right_result != -1 {
                return (right_result, 0);
            }

            // 都没找到，返回元素个数
            (-1, left_count + right_count + 1)
        }
    }
}
