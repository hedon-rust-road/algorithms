mod leetcode143;
mod leetcode147;
mod leetcode148;
mod leetcode19;
mod leetcode2;
mod leetcode203;
mod leetcode206;
mod leetcode21;
mod leetcode234;
mod leetcode24;
mod leetcode25;
mod leetcode328;
mod leetcode445;
mod leetcode61;
mod leetcode82;
mod leetcode83;
mod leetcode86;
mod leetcode92;

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
