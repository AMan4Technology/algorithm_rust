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
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn visit(root: Option<&Rc<RefCell<TreeNode>>>) -> String {
            let mut result = String::new();
            if let Some(root) = root {
                let root = root.borrow();
                let (left, right) = (visit(root.left.as_ref()), visit(root.right.as_ref()));
                result.push_str("l:");
                result.push_str(left.as_str());
                result.push(',');
                result.push_str(root.val.to_string().as_str());
                result.push(',');
                result.push_str("r:");
                result.push_str(right.as_str())
            } else {
                result.push(' ');
            }
            result
        }

        let (s, t) = (visit(s.as_ref()), visit(t.as_ref()));
        s.contains(t.as_str())
    }
}
