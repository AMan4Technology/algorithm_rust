// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::data_structure::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = std::i32::MIN;
        Self::max_of(root.as_ref(), &mut max);
        max
    }

    fn max_of(node: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let value = node.unwrap().borrow().val;
        let (left, right) = (
            Self::max_of(node.unwrap().borrow().left.as_ref(), max),
            Self::max_of(node.unwrap().borrow().right.as_ref(), max),
        );

        let mut curr = value;
        if left > 0 {
            curr += left;
        }
        if right > 0 {
            curr += right;
        }
        *max = (*max).max(curr);

        value.max(value + left).max(value + right)
    }
}
