//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
          val,
          left: None,
          right: None
        }
    }

    pub fn insert(&self, val: i32) {
        match self {
            None => self.val = Some(Rc::new(RefCell::new(TreeNode::new(10)))),
            Some(node) => {
                 match val < node.val {
                   true => node.left.into_raw.into_inner.insert(val);,
                   _ => node.right.into_raw.into_inner.insert(val);
                 }
            }
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    32    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_bst() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        root.insert(15);

        println!("root: {:?}", root);

        assert_eq!(range_sum_bst(root, 7, 15), 32);
    }
}
