use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        helper(&nums)
    }
}

fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }

    let mid_index = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid_index]);
    root.left = helper(&nums[..mid_index]);
    root.right = helper(&nums[mid_index + 1..]);
    Some(Rc::new(RefCell::new(root)))
}
