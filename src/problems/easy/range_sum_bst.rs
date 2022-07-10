// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    match root {
        None => 0,
        Some(rc) => {
            let val = rc.borrow().val;

            if low <= val && val <= high {
                val + range_sum_bst(rc.borrow().left.clone(), low, high)
                    + range_sum_bst(rc.borrow().right.clone(), low, high)
            } else {
                range_sum_bst(rc.borrow().left.clone(), low, high)
                    + range_sum_bst(rc.borrow().right.clone(), low, high)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_bst() {
        let three = TreeNode::new(3);
        let seven = TreeNode::new(7);
        let eighteen = TreeNode::new(18);
        let five = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(three))),
            right: Some(Rc::new(RefCell::new(seven))),
        };
        let fifteen = TreeNode {
            val: 15,
            left: None,
            right: Some(Rc::new(RefCell::new(eighteen))),
        };
        let root = TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(five))),
            right: Some(Rc::new(RefCell::new(fifteen))),
        };

        assert_eq!(range_sum_bst(Some(Rc::new(RefCell::new(root))), 7, 15), 32);
    }
}
