use crate::Solution;
use crate::data_structure::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut front = head.as_ref();
        for _ in 0..n {
            front = if let Some(node) = front { node.next.as_ref() } else { return head; };
        }
        if let None = front {
            return head.unwrap().next;
        }

        let mut result = head.clone();
        let mut behind = result.as_mut();
        while let Some(node) = front {
            if node.next.is_none() { break; }
            front = node.next.as_ref();
            behind = behind.unwrap().next.as_mut();
        }

        let behind = behind.unwrap();
        behind.next = if behind.next.as_ref().unwrap().next.is_none() {
            None
        } else {
            Some(behind.next.as_ref().unwrap().next.as_ref().unwrap().clone())
        };
        result
    }
}