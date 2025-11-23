mod leetcode100;
mod leetcode101;
mod leetcode104;
mod leetcode108;
mod leetcode110;
mod leetcode111;
mod leetcode112;
mod leetcode113;
mod leetcode129;
mod leetcode222;
mod leetcode226;
mod leetcode230;
mod leetcode235;
mod leetcode236;
mod leetcode257;
mod leetcode404;
mod leetcode437;
mod leetcode450;
mod leetcode98;

use std::{cell::RefCell, rc::Rc};

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

struct Solution;
