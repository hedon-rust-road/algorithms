use super::{ListNode, Solution};

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }
        if count == 0 {
            return head;
        }
        let k = k % count;
        rotate_right_help(head, k)
    }
}

fn rotate_right_help(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k == 0 {
        return head;
    }

    // 将结尾变到前面
    let mut tail = head.as_mut().unwrap();
    if tail.next.is_none() {
        return head;
    }
    while tail.next.as_ref().unwrap().next.as_ref().is_some() {
        tail = tail.next.as_mut().unwrap();
    }

    let mut new_head = tail.next.take();
    new_head.as_mut().unwrap().next = head;
    rotate_right_help(new_head, k - 1)
}
