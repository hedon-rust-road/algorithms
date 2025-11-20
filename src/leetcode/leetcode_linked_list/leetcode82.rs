use super::{ListNode, Solution};

impl Solution {
    pub fn delete_duplicates_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut tail = &mut dummy;

        // tail 是已经确定好的，现在是要确定 next 要不要保留
        while tail.next.is_some() {
            if tail.next.as_ref().unwrap().next.is_some()
                && tail.next.as_ref().unwrap().val
                    == tail.next.as_ref().unwrap().next.as_ref().unwrap().val
            {
                // 有重复，则过滤所有等于 val 的元素
                let val = tail.next.as_ref().unwrap().val;
                while tail.next.as_ref().is_some() && tail.next.as_ref().unwrap().val == val {
                    tail.next = tail.next.as_mut().unwrap().next.take();
                }
            } else {
                // 没有重复，移动到下一个
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
