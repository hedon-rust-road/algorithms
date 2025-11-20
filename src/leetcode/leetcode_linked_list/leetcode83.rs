use super::{ListNode, Solution};

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();
        while let Some(curr_node) = current {
            // 去掉所有相同的节点
            while curr_node.next.is_some() && curr_node.next.as_ref().unwrap().val == curr_node.val
            {
                curr_node.next = curr_node.next.as_mut().unwrap().next.take();
            }
            current = curr_node.next.as_mut();
        }
        head
    }
}
