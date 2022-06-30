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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (_, None) => false,
        (None, _) => false,
        (Some(a), Some(b)) => {
            a.borrow().val == b.borrow().val
                && is_same_tree(a.borrow().right.clone(), b.borrow().right.clone())
                && is_same_tree(a.borrow().left.clone(), b.borrow().left.clone())
        }
    }
}

pub fn is_same_tree_iterative(
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut stack = vec![];

    stack.push((p, q));

    while let Some(current_node) = stack.pop() {
        match current_node {
            (None, None) => continue,
            (_, None) => return false,
            (None, _) => return false,
            (Some(a), Some(b)) => {
                if a.borrow().val != b.borrow().val {
                    return false;
                }
                stack.push((a.borrow().right.clone(), b.borrow().right.clone()));
                stack.push((a.borrow().left.clone(), b.borrow().left.clone()));
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(1))))
            ),
            true
        );

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(2))))
            ),
            false
        );

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(1))))
            ),
            true
        );

        // with a right child
        let mut root = TreeNode::new(1);
        let right = TreeNode::new(3);
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut root2 = TreeNode::new(1);
        let right2 = TreeNode::new(3);
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );

        // with a left child
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(left)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(3);
        root2.left = Some(Rc::new(RefCell::new(left2)));

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );

        // with a left and right child
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(2);
        let right2 = TreeNode::new(3);
        root2.left = Some(Rc::new(RefCell::new(left2)));
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );
    }

    #[test]
    fn test_is_same_tree_iterative() {
        assert_eq!(
            is_same_tree_iterative(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(1))))
            ),
            true
        );

        assert_eq!(
            is_same_tree_iterative(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(2))))
            ),
            false
        );

        assert_eq!(
            is_same_tree_iterative(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(1))))
            ),
            true
        );

        // with a right child
        let mut root = TreeNode::new(1);
        let right = TreeNode::new(3);
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut root2 = TreeNode::new(1);
        let right2 = TreeNode::new(3);
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert_eq!(
            is_same_tree_iterative(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );

        // with a left child
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(left)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(3);
        root2.left = Some(Rc::new(RefCell::new(left2)));

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );

        // with a left and right child
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(2);
        let right2 = TreeNode::new(3);
        root2.left = Some(Rc::new(RefCell::new(left2)));
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert_eq!(
            is_same_tree(
                Some(Rc::new(RefCell::new(root))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            true
        );
    }
}
