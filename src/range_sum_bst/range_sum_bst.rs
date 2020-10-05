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
            right: None,
        }
    }

    pub fn insert(
        root: Option<Rc<RefCell<TreeNode>>>,
        new_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(new_val)))),
            Some(ptr) => {
                let val = ptr.borrow().val.clone();

                match new_val < val {
                    true => {
                        let left = TreeNode::insert(ptr.borrow().left.clone(), new_val);
                        ptr.borrow_mut().left = left;
                    }
                    _ => {
                        let right = TreeNode::insert(ptr.borrow().right.clone(), new_val);
                        ptr.borrow_mut().right = right;
                    }
                }
                Some(ptr.clone())
            }
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
/*
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    32
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_left() {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let mut root_node = TreeNode::new(10);
        root_node.left = left;

        let root = Some(Rc::new(RefCell::new(root_node)));

        let actual = TreeNode::insert(Some(Rc::new(RefCell::new(TreeNode::new(10)))), 5);

        assert_eq!(actual, root);
    }

    #[test]
    fn test_insert_right() {
        let right = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let mut root_node = TreeNode::new(10);
        root_node.right = right;

        let root = Some(Rc::new(RefCell::new(root_node)));

        assert_eq!(
            TreeNode::insert(Some(Rc::new(RefCell::new(TreeNode::new(10)))), 15),
            root
        );
    }

    #[test]
    fn test_insert_lower() {
        let grandchild = Some(Rc::new(RefCell::new(TreeNode::new(13))));
        let mut child_node = TreeNode::new(10);
        child_node.right = grandchild;

        let child = Some(Rc::new(RefCell::new(child_node)));
        let mut root_node = TreeNode::new(15);
        root_node.left = child;

        let root = Some(Rc::new(RefCell::new(root_node)));

        assert_eq!(
            TreeNode::insert(
                TreeNode::insert(Some(Rc::new(RefCell::new(TreeNode::new(15)))), 10),
                13
            ),
            root
        )
    }
}

/*
    #[test]
    fn test_range_sum_bst() {
        assert_eq!(range_sum_bst(root, 7, 15), 32);
    }
*/
