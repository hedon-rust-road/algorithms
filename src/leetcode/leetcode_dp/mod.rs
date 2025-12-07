use std::{cell::RefCell, rc::Rc};

mod knapsack01;
mod leetcode1143;
mod leetcode120;
mod leetcode139;
mod leetcode198;
mod leetcode213;
mod leetcode279;
mod leetcode300;
mod leetcode309;
mod leetcode322;
mod leetcode337;
mod leetcode343;
mod leetcode376;
mod leetcode377;
mod leetcode416;
mod leetcode42;
mod leetcode474;
mod leetcode494;
mod leetcode62;
mod leetcode63;
mod leetcode64;
mod leetcode70;
mod leetcode91;

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
