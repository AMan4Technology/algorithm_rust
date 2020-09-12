use crate::{data_structure::ListNode, Solution};

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut slow, mut fast) = (head.as_ref(), head.as_ref());
        while let Some(fast_node) = fast {
            if fast_node.next.is_none() {
                break;
            }
            fast = fast_node.next.as_ref().unwrap().next.as_ref();
            slow = slow.unwrap().next.as_ref();
        }
        Some(slow.unwrap().clone())
    }
}
