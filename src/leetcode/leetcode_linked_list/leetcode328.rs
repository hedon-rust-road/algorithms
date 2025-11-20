use super::{ListNode, Solution};

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy = Box::new(ListNode { val: 0, next: None });
        let mut odd_tail = &mut odd_dummy;
        let mut even_dummy = Box::new(ListNode { val: 0, next: None });
        let mut even_tail = &mut even_dummy;

        let mut i = 1;
        while let Some(mut node) = head {
            head = node.next.take();
            if i % 2 == 1 {
                odd_tail.next = Some(node);
                odd_tail = odd_tail.next.as_mut().unwrap();
            } else {
                even_tail.next = Some(node);
                even_tail = even_tail.next.as_mut().unwrap();
            }
            i += 1;
        }

        odd_tail.next = even_dummy.next;
        odd_dummy.next
    }
}
