use super::{ListNode, Solution};

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        // 找到起始点
        let mut start_prev = &mut dummy;
        for _ in 0..left - 1 {
            let next = start_prev.next.as_mut().unwrap();
            start_prev = next;
        }
        let mut start = start_prev.next.take();

        // 反转 [left, right]
        let mut pre: Option<Box<ListNode>> = None;
        for _ in left..=right {
            let mut node = start.unwrap();
            let next = node.next.take();
            node.next = pre;
            pre = Some(node);
            start = next;
        }

        let new_start = pre;
        let end_next = start;

        // 串联前半部分起来
        start_prev.next = new_start;

        // 找到新的结尾，串起来后半部分
        let mut tail = &mut start_prev.next;
        while tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = end_next;

        dummy.next
    }
}
