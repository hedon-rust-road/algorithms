use super::{ListNode, Solution};

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut tail = &mut dummy;

        for _ in 0..(count - n) {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = tail.next.as_mut().unwrap().next.take();
        dummy.next
    }
}
