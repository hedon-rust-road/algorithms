use super::{ListNode, Solution};

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // if list1.is_none() {
        //     list2
        // } else if list2.is_none() {
        //     list1
        // } else if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
        //     let next = list1.as_mut().unwrap().next.take();
        //     list1.as_mut().unwrap().next = Self::merge_two_lists(next, list2);
        //     list1
        // } else {
        //     let next = list2.as_mut().unwrap().next.take();
        //     list2.as_mut().unwrap().next = Self::merge_two_lists(next, list1);
        //     list2
        // }

        let mut dummy = Box::new(ListNode {
            val: 0,
            next: list1,
        });

        while list2.is_some() {
            let next = list2.as_mut().unwrap().next.take();
            find_position_and_insert(&mut dummy, list2);
            list2 = next;
        }

        dummy.next
    }
}

fn find_position_and_insert(dummy: &mut Box<ListNode>, node: Option<Box<ListNode>>) {
    let mut pre = dummy;

    while pre.next.is_some() && pre.next.as_ref().unwrap().val <= node.as_ref().unwrap().val {
        pre = pre.next.as_mut().unwrap();
    }

    let next = pre.as_mut().next.take();
    pre.next = node;
    pre.next.as_mut().unwrap().next = next;
}
