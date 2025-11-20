use super::{ListNode, Solution};

impl Solution {
    pub fn add_two_numbers_2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_stack = build_stack(l1);
        let l2_stack = build_stack(l2);

        add_two_numbers_2_help(l1_stack, l2_stack)
    }
}

fn add_two_numbers_2_help(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut carry = 0;

    loop {
        let l1_val = l1.pop();
        let l2_val = l2.pop();

        if carry == 0 && l1_val.is_none() && l2_val.is_none() {
            break;
        }

        let mut val = carry + l1_val.unwrap_or(0) + l2_val.unwrap_or(0);
        carry = val / 10;
        val = val % 10;

        let head = Some(Box::new(ListNode {
            val: val,
            next: result,
        }));

        result = head
    }

    result
}

fn build_stack(l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut tmp = l;
    let mut result = vec![];
    while let Some(node) = tmp {
        result.push(node.val);
        tmp = node.next;
    }
    result
}
