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
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let (node, _) = Self::revocer(s.as_str(), 0);
        if let Some(node) = node {
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }

    fn revocer(s: &str, deep: usize) -> (Option<TreeNode>, &str) {
        let index = s.find(char::is_numeric);
        if index.is_none() || deep != index.unwrap() {
            return (None, s);
        }

        let index = index.unwrap();
        let s = &s[index..];
        let end = if let Some(index) = s.find('-') {
            index
        } else {
            s.len()
        };
        let mut node = TreeNode::new(s[..end].parse().unwrap());

        let s = &s[end..];
        if s.len() == 0 {
            return (Some(node), s);
        }
        let (left, s) = Self::revocer(s, deep + 1);
        node.left = if let Some(left) = left {
            Some(Rc::new(RefCell::new(left)))
        } else {
            None
        };

        if s.len() == 0 {
            return (Some(node), s);
        }
        let (right, s) = Self::revocer(s, deep + 1);
        node.right = if let Some(right) = right {
            Some(Rc::new(RefCell::new(right)))
        } else {
            None
        };

        (Some(node), s)
    }
}
