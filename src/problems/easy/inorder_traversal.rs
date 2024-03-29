use std::cell::RefCell;
use std::rc::Rc;

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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(rc) => {
            let node = rc.borrow();
            let val = node.val;

            let mut left = inorder_traversal(node.left.clone());
            let right = inorder_traversal(node.right.clone());

            left.push(val);
            left.extend(right);
            left
        }
        None => {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left,
            right,
        })));

        assert_eq!(inorder_traversal(root), vec![1, 2, 3]);

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let result = inorder_traversal(root);
        println!("result: {:?}", result);
        assert_eq!(result, vec![4, 2, 5, 1, 3]);
    }
}
