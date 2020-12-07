use std::cell::RefCell;
use std::rc::Rc; //Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Tree>,
    pub right: Option<Tree>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tree(Rc<RefCell<TreeNode>>);

impl Tree {
    #[inline]
    pub fn new(val: i32) -> Self {
        Tree(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }

    pub fn value(&self) -> i32 {
        self.0.borrow().val.clone()
    }

    pub fn insert(&self, new_val: i32) {
        match new_val < self.value() {
            true => {
                let mut ptr = self.0.borrow_mut();

                match &ptr.left {
                    None => ptr.left = Some(Tree::new(new_val)),
                    Some(tree) => tree.insert(new_val),
                }
            }
            _ => {
                let mut ptr = self.0.borrow_mut();

                match &ptr.right {
                    None => ptr.right = Some(Tree::new(new_val)),
                    Some(tree) => tree.insert(new_val),
                }
            }
        }
    }
}
/*
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    match root {
        None => 0,
        Some(ptr) => {
            let val = ptr.borrow().val.clone();
            let left = ptr.borrow().left.clone();
            let right = ptr.borrow().right.clone();

            match (l <= val, r >= val) {
                (true, true) => val + range_sum_bst(left, l, r) + range_sum_bst(right, l, r),
                _ => range_sum_bst(left, l, r) + range_sum_bst(right, l, r),
            }
        }
    }
}
*/
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Tree::new(42),
            Tree(Rc::new(RefCell::new(TreeNode {
                val: 42,
                left: None,
                right: None
            })))
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(Tree::new(42).value(), 42);
    }

    #[test]
    fn test_insert_left() {
        let actual = Tree::new(42);
        actual.insert(5);

        assert_eq!(
            actual,
            Tree(Rc::new(RefCell::new(TreeNode {
                val: 42,
                left: Some(Tree::new(5)),
                right: None
            })))
        )
    }

    #[test]
    fn test_insert_right() {
        let actual = Tree::new(42);
        actual.insert(43);

        assert_eq!(
            actual,
            Tree(Rc::new(RefCell::new(TreeNode {
                val: 42,
                left: None,
                right: Some(Tree::new(43))
            })))
        )
    }

    #[test]
    fn test_insert_lower() {
        let actual = Tree::new(15);
        actual.insert(10);
        actual.insert(13);

        assert_eq!(
            actual,
            Tree(Rc::new(RefCell::new(TreeNode {
                val: 15,
                right: None,
                left: Some(Tree(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: None,
                    right: Some(Tree::new(13))
                }))))
            })))
        )
    }
}

/*
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
*/
