use std::collections::BinaryHeap;

use super::{ListNode, Solution};

struct NodeWrapper {
    node: Box<ListNode>,
}

impl PartialEq for NodeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.node.val == other.node.val
    }
}

impl Eq for NodeWrapper {}

impl PartialOrd for NodeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 注意这里的顺序：other.cmp(self)
        // Rust 的 BinaryHeap 是大顶堆。
        // 为了让“值小的节点”排在堆顶，我们必须告诉堆：
        // "如果 A 的值比 B 小，那么 A 其实比 B '大' (优先级更高)"
        other.node.val.cmp(&self.node.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for list in lists {
            if let Some(list) = list {
                min_heap.push(NodeWrapper { node: list });
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while let Some(NodeWrapper { mut node }) = min_heap.pop() {
            if let Some(next) = node.next.take() {
                min_heap.push(NodeWrapper { node: next });
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
