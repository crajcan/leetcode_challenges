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

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        None => false,
        Some(n) => match (n.borrow().left.clone(), n.borrow().right.clone()) {
            (None, None) => n.borrow().val == target_sum,
            (left, None) => has_path_sum(left, target_sum - n.borrow().val),
            (None, right) => has_path_sum(right, target_sum - n.borrow().val),
            (left, right) => {
                has_path_sum(left, target_sum - n.borrow().val)
                    || has_path_sum(right, target_sum - n.borrow().val)
            }
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_path_sum() {
        let one = TreeNode::new(1);
        let two = TreeNode::new(2);
        let seven = TreeNode::new(7);
        let thirteen = TreeNode::new(13);
        let four = TreeNode {
            val: 4,
            left: None,
            right: Some(Rc::new(RefCell::new(one))),
        };
        let eleven = TreeNode {
            val: 11,
            left: Some(Rc::new(RefCell::new(seven))),
            right: Some(Rc::new(RefCell::new(two))),
        };
        let eight = TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(thirteen))),
            right: Some(Rc::new(RefCell::new(four))),
        };
        let other_four = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(eleven))),
            right: None,
        };
        let five = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(other_four))),
            right: Some(Rc::new(RefCell::new(eight))),
        };

        assert_eq!(has_path_sum(Some(Rc::new(RefCell::new(five))), 22), true);
    }
}
