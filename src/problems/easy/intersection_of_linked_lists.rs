use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn lists_intersection(
    left: Option<Rc<RefCell<ListNode>>>,
    right: Option<Rc<RefCell<ListNode>>>,
    step_left: bool,
) -> Option<Rc<RefCell<ListNode>>> {
    match (left, right) {
        (None, _) => None,
        (_, None) => None,
        (Some(left_rc), Some(right_rc)) => {
            if Rc::ptr_eq(&left_rc,&right_rc) {
                Some(left_rc.clone())
            } else {
                if step_left {
                    lists_intersection(left_rc.borrow().next.clone(), Some(right_rc.clone()), false)
                } else {
                    lists_intersection(Some(left_rc.clone()), right_rc.borrow().next.clone(), true)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lists_intersection() {
        let tail = Some(Rc::new(RefCell::new(ListNode::new(0))));
        assert_eq!(lists_intersection(tail, None, true), None);

        let left = Some(Rc::new(RefCell::new(ListNode::new(0))));
        let right = Some(Rc::new(RefCell::new(ListNode::new(0))));
        assert_eq!(lists_intersection(left, right, true), None);

        let tail = Some(Rc::new(RefCell::new(ListNode::new(0))));
        let intersection = Some(Rc::new(RefCell::new(ListNode {
            val: 0,
            next: tail.clone(),
        })));

        let left_connection = Some(Rc::new(RefCell::new(ListNode {
            val: 1,
            next: intersection.clone(),
        })));
        let right_connection = Some(Rc::new(RefCell::new(ListNode {
            val: 2,
            next: intersection.clone(),
        })));

        let left = Some(Rc::new(RefCell::new(ListNode {
            val: 1,
            next: left_connection.clone(),
        })));
        let right = Some(Rc::new(RefCell::new(ListNode {
            val: 2,
            next: intersection.clone(),
        })));

        assert_eq!(
            lists_intersection(left.clone(), right.clone(), true),
            intersection.clone()
        );
    }
}
