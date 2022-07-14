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

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(rc) => {
            let node = rc.borrow();
            let val = node.val;
            println!("refcell.borrow().value: {}", val);

            let left = preorder_traversal(node.left.as_ref().map(Rc::clone));
            let mut center = vec![val];
            let right = preorder_traversal(node.right.clone());

            center.extend(left);
            center.extend(right);
            center
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
    fn test_preorder_traversal() {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left,
            right,
        })));

        assert_eq!(preorder_traversal(root), vec![2, 1, 3]);
    }
}
