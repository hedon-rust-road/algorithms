use super::{ListNode, Solution};

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return true;
        }

        let (mut left, right) = split(head);
        let mut right = reverse(right);

        while let Some(mut node1) = left {
            left = node1.next.take();
            if let Some(mut node2) = right {
                right = node2.next.take();
                if node1.val != node2.val {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }
}

fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut count = 0;
    let mut curr = &head;
    while let Some(node) = curr {
        count += 1;
        curr = &node.next;
    }

    // 找到中点，进行拆分
    let mut curr = &mut head;
    for _ in 0..(count / 2 - 1) {
        curr = &mut curr.as_mut().unwrap().next;
    }

    let second_half = curr.as_mut().unwrap().next.take();
    (head, second_half)
}

fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}
