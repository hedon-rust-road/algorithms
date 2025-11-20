use super::{ListNode, Solution};

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let (left, right) = split(head);
        merge_two_sorted_list(Self::sort_list(left), Self::sort_list(right))
    }
}

fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut len = 0;
    let mut cur = &head;
    while let Some(node) = cur {
        len += 1;
        cur = &node.next;
    }

    if len <= 1 {
        return (head, None);
    }

    let mut cur = &mut head;
    for _ in 0..(len / 2 - 1) {
        cur = &mut cur.as_mut().unwrap().next;
    }

    let second_half = cur.as_mut().unwrap().next.take();
    (head, second_half)
}

fn merge_two_sorted_list(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut cur = &mut dummy;

    while list1.is_some() && list2.is_some() {
        let mut node: Box<ListNode>;
        if list1.as_deref().unwrap().val < list2.as_deref().unwrap().val {
            node = list1.unwrap();
            list1 = node.next.take();
        } else {
            node = list2.unwrap();
            list2 = node.next.take();
        }

        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
    }

    if list1.is_some() {
        cur.next = list1;
    }

    if list2.is_some() {
        cur.next = list2;
    }

    dummy.next
}
