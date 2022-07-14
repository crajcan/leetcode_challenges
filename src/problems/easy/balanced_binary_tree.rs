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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_balanced_helper(root).is_some()
}

pub fn is_balanced_helper(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    println!("\nroot: {:#?}\n", root);
    match root {
        None => Some(0),
        Some(node) => {
            let left_child_height = is_balanced_helper(node.borrow().left.clone());
            let right_child_height = is_balanced_helper(node.borrow().right.clone());
            match (left_child_height, right_child_height) {
                (None, _) => None,
                (_, None) => None,
                (Some(left_height), Some(right_height)) => {
                    if (left_height - right_height).abs() > 1 {
                        None
                    } else {
                        Some(1 + *[left_height, right_height].iter().max().unwrap())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_balanced() {
        assert_eq!(is_balanced(None), true);
        assert_eq!(
            is_balanced(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))),
            true
        );
        assert_eq!(
            is_balanced(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: None
            })))),
            true
        );
        assert_eq!(
            is_balanced(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            })))),
            true
        );
        assert_eq!(
            is_balanced(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))),
            true
        );
        assert_eq!(
            is_balanced(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: None
                    }))),
                    right: None
                })))
            })))),
            false
        );
    }
}
