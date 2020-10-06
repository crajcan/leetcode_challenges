use std::cell::RefCell;
use std::rc::Rc;

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
                let val = ptr.borrow().val;

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

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    match root {
        None => 0,
        Some(ptr) => {
            let val = ptr.borrow().val.clone();

            match (l <= val, r >= val) {
                (true, true) => {
                    val + range_sum_bst(ptr.borrow().left.clone(), l, r)
                        + range_sum_bst(ptr.borrow().right.clone(), l, r)
                }
                _ => {
                    range_sum_bst(ptr.borrow().left.clone(), l, r)
                        + range_sum_bst(ptr.borrow().right.clone(), l, r)
                }
            }
        }
    }
}

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

    #[test]
    fn test_range_sum_bst() {
        let root = None;
        let tree = vec![10, 5, 15, 3, 7, 18]
            .iter()
            .fold(root, |r, val| TreeNode::insert(r, *val));

        assert_eq!(range_sum_bst(tree.clone(), 7, 15), 32);

        let next_tree = vec![1, 6]
            .iter()
            .fold(tree, |t, val| TreeNode::insert(t, *val));

        assert_eq!(range_sum_bst(next_tree, 6, 10), 23);
    }
}
