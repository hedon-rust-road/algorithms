use super::{ListNode, Solution};

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node1) => match node1.next.take() {
                Some(mut node2) => {
                    node1.next = Self::swap_pairs(node2.next.take());
                    node2.next = Some(node1);
                    Some(node2)
                }
                None => Some(node1),
            },
            None => None,
        }

        // let mut dummy = Box::new(ListNode { val: 0, next: head });
        // let mut prev = &mut dummy;

        // // prev -> next -> next_next
        // // prev -> next_next -> next
        // // prev = next

        // // prev
        // // next
        // // next_next ->
        // while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
        //     let mut next = prev.next.take().unwrap();
        //     let mut next_next = next.next.take().unwrap();
        //     let next_next_next = next_next.next.take();

        //     next.next = next_next_next;
        //     next_next.next = Some(next);
        //     prev.next = Some(next_next);

        //     prev = prev.next.as_mut().unwrap().next.as_mut().unwrap()
        // }

        // dummy.next
    }
}
