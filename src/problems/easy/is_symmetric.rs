use std::{cell::RefCell, rc::Rc};

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

pub fn is_symmetric_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![];

    match root {
        None => return true,
        Some(node) => stack.push((node.borrow().left.clone(), node.borrow().right.clone())),
    }

    while let Some(node) = stack.pop() {
        match node {
            (None, None) => return true,
            (None, _) => return false,
            (_, None) => return false,
            (Some(a), Some(b)) => {
                if a.borrow().val != b.borrow().val {
                    return false;
                } else {
                    stack.push((a.borrow().left.clone(), b.borrow().right.clone()));
                    stack.push((a.borrow().right.clone(), b.borrow().left.clone()));
                }
            }
        }
    }

    true
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(rc) => is_symmetric_helper(rc.borrow().left.clone(), rc.borrow().right.clone()),
    }
}

pub fn is_symmetric_helper(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (None, _) => false,
        (_, None) => false,
        (Some(a), Some(b)) => {
            if a.borrow().val != b.borrow().val {
                false
            } else {
                is_symmetric_helper(a.borrow().left.clone(), b.borrow().right.clone())
                    && is_symmetric_helper(a.borrow().right.clone(), b.borrow().left.clone())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_symmetric() {
        assert_eq!(is_symmetric(None), true);
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            true
        );
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
            })))),
            true
        );
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            })))),
            false
        );

        //test two levels
        assert_eq!(
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                })))
            })))),
            true
        );
    }

    #[test]
    fn test_is_symmetric_iterative() {
        assert_eq!(is_symmetric_iterative(None), true);
        assert_eq!(
            is_symmetric_iterative(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            true
        );
        assert_eq!(
            is_symmetric_iterative(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
            })))),
            true
        );
        assert_eq!(
            is_symmetric_iterative(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            })))),
            false
        );

        //test two levels
        assert_eq!(
            is_symmetric_iterative(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                })))
            })))),
            true
        );
    }
}
