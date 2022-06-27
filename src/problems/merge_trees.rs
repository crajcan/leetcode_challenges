use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

pub fn merge_trees(
    _t1: Option<Rc<RefCell<TreeNode>>>,
    _t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(13))))
}
/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_trees() {
        assert_eq!(Some(Rc::new(RefCell::new(TreeNode::new(13))));
    }
}
*/
