
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

pub fn sorted_array_to_bst_iterative(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    match nums[..] {
        [] => None,
        [x] => Some(Rc::new(RefCell::new(TreeNode::new(x)))),
        _ => {
            let mut stack = vec![];
            let mid = nums.len() / 2;

            let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            stack.push((Rc::clone(&root), &nums[..]));

            while let Some((node, nums)) = stack.pop() {
                let n = &(*node);

                match nums {
                    [] => (),
                    [_x] => (),
                    [x, _y] => {
                        let child = Rc::new(RefCell::new(TreeNode::new(*x)));
                        n.borrow_mut().left = Some(child.clone());
                    }
                    [x, _y, z] => {
                        let left_child = Rc::new(RefCell::new(TreeNode::new(*x)));
                        let right_child = Rc::new(RefCell::new(TreeNode::new(*z)));
                        n.borrow_mut().left = Some(left_child.clone());
                        n.borrow_mut().right = Some(right_child.clone());

                        stack.push((left_child, &nums[..1]));
                        stack.push((right_child, &nums[2..]));
                    }
                    _ => {
                        let mid = nums.len() / 2;
                        let lefts = &nums[..mid];
                        let rights = &nums[(mid + 1)..];
                        let left_mid = lefts[lefts.len() / 2];
                        let right_mid = rights[rights.len() / 2];

                        let left_child = Rc::new(RefCell::new(TreeNode::new(left_mid)));
                        let right_child = Rc::new(RefCell::new(TreeNode::new(right_mid)));

                        n.borrow_mut().left = Some(left_child.clone());
                        n.borrow_mut().right = Some(right_child.clone());

                        stack.push((left_child, &nums[..mid]));
                        stack.push((right_child, &nums[(mid + 1)..]));
                    }
                }
            }

            Some(root)
        }
    }
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    to_tree(&nums[..])
}

pub fn to_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    match nums {
        [] => None,
        [x] => Some(Rc::new(RefCell::new(TreeNode::new(*x)))),
        _ => {
            let midi = (nums.len()) / 2;
            let mid = nums[midi];
            let mut node = TreeNode::new(mid);
            node.left = to_tree(&nums[..midi]);
            node.right = to_tree(&nums[(midi + 1)..]);

            Some(Rc::new(RefCell::new(node)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_sorted_array_to_bst() {
        assert_eq!(
            sorted_array_to_bst(vec![-10, -3, -1, 0, 5, 9]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -10,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -1,
                        left: None,
                        right: None
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                    right: None,
                })))
            })))
        );
    }

    #[test]
    fn test_sorted_array_to_bst_iterative() {
        assert_eq!(
            sorted_array_to_bst_iterative(vec![-10, -3, -1, 0, 5, 9]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -10,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -1,
                        left: None,
                        right: None
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                    right: None,
                })))
            })))
        );
    }
}
