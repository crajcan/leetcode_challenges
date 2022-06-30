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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_max = max_depth(node.borrow().left.clone());
            let right_max = max_depth(node.borrow().right.clone());

            1 + [left_max, right_max].iter().max().unwrap()
        }
    }
}

pub fn max_depth_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = vec![];
    let mut max = 0;

    match root {
        None => return 0,
        Some(node) => stack.push((Some(node), 0)),
    }

    while let Some(node) = stack.pop() {
        match node {
            (None, d) => {
                if d > max {
                    max = d;
                }
            }
            (Some(node), d) => {
                let depth = d + 1;

                stack.push((node.borrow().left.clone(), depth));
                stack.push((node.borrow().right.clone(), depth));
            }
        }
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(max_depth(None), 0);
        assert_eq!(max_depth(Some(Rc::new(RefCell::new(TreeNode::new(1))))), 1);
        assert_eq!(
            max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))),
            2
        );
        assert_eq!(
            max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            })))),
            3
        );
    }

    #[test]
    fn test_max_depth_iterative() {
        assert_eq!(max_depth_iterative(None), 0);
        assert_eq!(
            max_depth_iterative(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            1
        );
        assert_eq!(
            max_depth_iterative(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))),
            2
        );
        assert_eq!(
            max_depth_iterative(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            })))),
            3
        );
    }
}
