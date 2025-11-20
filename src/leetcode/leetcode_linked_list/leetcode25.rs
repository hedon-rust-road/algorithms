use super::{ListNode, Solution};

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        // 检测是否满足 k 个节点
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            if count == k {
                break;
            }
            curr = &node.next;
        }

        if count < k {
            return head;
        }

        // 反转前 k 个节点
        let mut prev = None;
        let mut curr = head;
        for _ in 0..k {
            if let Some(mut node) = curr {
                curr = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
        }

        // 递归处理剩余部分
        if let Some(mut new_head) = prev {
            // 找到新的结尾
            let mut tail = &mut new_head;
            while tail.as_ref().next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
            tail.as_mut().next = Self::reverse_k_group(curr, k);
            Some(new_head)
        } else {
            prev
        }
    }
}
