use super::{ListNode, Solution};

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let taken = head.take();
        let (left, right) = split(taken);
        let right = reverse(right);
        let result = merge(left, right);
        *head = result;
    }
}

fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    // 统计个数
    let mut count = 0;
    let mut curr = &head;
    while let Some(node) = curr {
        count += 1;
        curr = &node.next;
    }
    if count <= 1 {
        return (head, None);
    }

    // 找到中点
    let mut curr = &mut head;
    for _ in 0..(count / 2 - 1) {
        curr = &mut curr.as_mut().unwrap().next;
    }

    if curr.is_none() {
        return (head, None);
    }

    let second_half = curr.as_mut().unwrap().next.take();
    (head, second_half)
}

fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node)
    }
    prev
}

fn merge(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });

    let mut i = 0;
    let mut curr = &mut dummy;
    while list1.is_some() && list2.is_some() {
        let mut node: Box<ListNode>;
        if i % 2 == 0 {
            node = list1.unwrap();
            list1 = node.as_mut().next.take();
        } else {
            node = list2.unwrap();
            list2 = node.as_mut().next.take();
        }
        i += 1;
        curr.next = Some(node);
        curr = curr.next.as_mut().unwrap();
    }

    if list1.is_some() {
        curr.next = list1;
    }
    if list2.is_some() {
        curr.next = list2;
    }
    dummy.next
}
