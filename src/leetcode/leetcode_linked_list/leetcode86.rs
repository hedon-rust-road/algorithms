use super::{ListNode, Solution};

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut small = Box::new(ListNode { val: 0, next: None });
        let mut small_end = &mut small;
        let mut big = Box::new(ListNode { val: 0, next: None });
        let mut big_end = &mut big;

        while let Some(mut node) = head {
            let next = node.next.take();
            if node.val < x {
                small_end.next = Some(node);
                small_end = small_end.next.as_mut().unwrap();
            } else {
                big_end.next = Some(node);
                big_end = big_end.next.as_mut().unwrap();
            }
            head = next;
        }

        small_end.next = big.next;
        small.next
    }
}
