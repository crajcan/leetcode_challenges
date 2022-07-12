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
pub fn lca(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(rc) => {
            let left = rc.borrow().left.clone();
            let right = rc.borrow().right.clone();
            let val = rc.borrow().val.clone();

            use std::cmp::Ordering::{Greater, Less};
            match (p.cmp(&val), q.cmp(&val)) {
                (Less, Less) => lca(left, p, q),
                (Greater, Greater) => lca(right, p, q),
                (_, _) => Some(Rc::clone(&rc)),
            }
        }
    }
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.unwrap().borrow().val;
    let q = q.unwrap().borrow().val;

    lca(root, p, q)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let right_left = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })));

        let right_right = Some(Rc::new(RefCell::new(TreeNode {
            val: 11,
            left: None,
            right: None,
        })));

        let right = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: right_left.clone(),
            right: right_right.clone(),
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: right.clone(),
        })));

        assert_eq!(lowest_common_ancestor(root, right_left, right_right), right);

        let left_left = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));

        let left_right = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));

        let left = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: left_left.clone(),
            right: left_right.clone(),
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: left.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(lowest_common_ancestor(root, left_left, left_right), left);

        let left = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));

        let right = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: left.clone(),
            right: right.clone(),
        })));

        assert_eq!(lowest_common_ancestor(root.clone(), left, right), root);
    }
}
