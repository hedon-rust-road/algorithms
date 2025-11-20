use super::{ListNode, Solution};

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut tail = &mut dummy;

        while let Some(ref mut node) = tail.next {
            if node.val == val {
                tail.next = node.next.take();
            } else {
                tail = tail.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
