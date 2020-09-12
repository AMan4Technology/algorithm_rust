use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::{
    Solution,
    data_structure::TreeNode,
};

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn visit(result: &mut i32, root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let root = root.borrow();
                let (left, right) = (visit(result, root.left.as_ref()) + 1, visit(result, root.right.as_ref()) + 1);
                *result = (*result).max(left + right);
                return left.max(right);
            }
            -1
        }

        let mut result = 0;
        visit(&mut result, root.as_ref());
        result
    }
}