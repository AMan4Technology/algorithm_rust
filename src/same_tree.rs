use crate::{data_structure::TreeNode, Solution};
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn equal(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    if p.val == q.val
                        && equal(p.left.as_ref(), q.left.as_ref())
                        && equal(p.right.as_ref(), q.right.as_ref())
                    {
                        true
                    } else {
                        false
                    }
                }
                (None, None) => true,
                _ => false,
            }
        }

        equal(p.as_ref(), q.as_ref())
    }
}
