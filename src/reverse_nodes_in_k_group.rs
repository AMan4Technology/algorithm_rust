use crate::{data_structure::ListNode, Solution};

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }

        let mut h = head.as_ref().unwrap().clone();
        let mut tail = &mut h;
        let (mut prev, mut curr) = (head.as_ref(), head.as_ref());
        let mut count = 0;
        while let Some(curr_node) = curr {
            curr = curr_node.next.as_ref();
            count += 1;
            if count != k {
                continue;
            }
            tail.next = Self::reverse(prev.unwrap().clone(), k);
            count = 0;
            prev = curr_node.next.as_ref();
            for _ in 0..k {
                tail = tail.next.as_mut().unwrap();
            }
        }
        if count != 0 {
            tail.next = Some(prev.unwrap().clone());
        }
        h.next
    }

    fn reverse(head: Box<ListNode>, len: i32) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        for _ in 0..len - 1 {
            let next = curr.next.unwrap();
            curr.next = prev;
            prev = Some(curr);
            curr = next;
        }
        curr.next = prev;
        Some(curr)
    }
}
