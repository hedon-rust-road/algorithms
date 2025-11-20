use super::{ListNode, Solution};

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));

        while let Some(mut node) = head {
            head = node.next.take();
            find_position_and_insert(&mut dummy, node);
        }

        dummy.next
    }
}

fn find_position_and_insert(dummy: &mut Box<ListNode>, mut node: Box<ListNode>) {
    let mut pre = dummy;
    while pre.next.as_ref().is_some_and(|n| n.val < node.val) {
        pre = pre.next.as_mut().unwrap();
    }
    node.next = pre.next.take();
    pre.next = Some(node);
}
