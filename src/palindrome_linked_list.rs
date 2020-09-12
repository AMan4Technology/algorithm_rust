use crate::data_structure::ListNode;
use crate::Solution;

impl Solution {
    pub fn is_palindrome_link(head: Option<Box<ListNode>>) -> bool {
        if let Some(head) = head.as_ref() {
            if head.next == None {
                return true;
            }
        } else {
            return true;
        }

        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head.unwrap();
        let curr_node = curr.clone();
        let mut fast = curr_node.next.as_ref();
        let mut count = 1;
        while let Some(fast_node) = fast {
            count += 1;
            let mut next = curr.next.unwrap();
            curr.next = prev;
            prev = Some(curr);
            curr = next;
            if fast_node.next.as_ref().is_none() {
                break;
            }
            count += 1;
            fast = fast_node.next.as_ref().unwrap().next.as_ref();
        }

        let mut left = prev;
        let mut right = if count & 1 != 0 {
            curr.next
        } else {
            Some(curr)
        };
        loop {
            match (left, right) {
                (Some(left_node), Some(right_node)) => {
                    if left_node.val != right_node.val {
                        break false;
                    }
                    left = left_node.next;
                    right = right_node.next;
                }
                (None, None) => {
                    break true;
                }
                _ => {
                    break false;
                }
            }
        }
    }
}
