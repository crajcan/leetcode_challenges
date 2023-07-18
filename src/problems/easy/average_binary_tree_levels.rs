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
      right: None
    }
  }
}

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut levels = vec![];

    dfs_helper(root, &mut levels, 1);

     
    levels.iter().map(|level| level.iter().sum::<f64>() as f64 / level.len() as f64).collect()
}

pub fn dfs_helper(root: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<f64>>, level: usize) {
    match root {
        None => (),       
        Some(rc) =>  {
            let node = rc.borrow();

            if levels.len() < level {
                println!("got here");
                levels.push(vec![]);
            }
            levels[level - 1].push(node.val as f64);

            dfs_helper(node.left.clone(), levels, level + 1);
            dfs_helper(node.right.clone(), levels, level + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_average_of_levels() {
        let three = TreeNode::new(3);
        let seven = TreeNode::new(7);
        let eighteen = TreeNode::new(18);
        let five = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(three))),
            right: Some(Rc::new(RefCell::new(seven))),
        };
        let fifteen = TreeNode {
            val: 15,
            left: None,
            right: Some(Rc::new(RefCell::new(eighteen))),
        };
        let root = TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(five))),
            right: Some(Rc::new(RefCell::new(fifteen))),
        };

        assert_eq!(average_of_levels(Some(Rc::new(RefCell::new(root)))), vec![10.0, 10.0, 9.333333333333334])
    }
}