use std::cell::RefCell;
use std::rc::Rc;

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

pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut tortoise = head.clone();
    let mut hare = match head {
        Some(rc) => rc.borrow().next.clone(),
        None => return false,
    };

    while hare.is_some() {
        if tortoise == hare {
            return true;
        }

        tortoise = match tortoise {
            Some(rc) => rc.borrow().next.clone(),
            None => return false,
        };

        hare = match hare {
            Some(rc) => match rc.borrow().next.clone() {
                Some(rc) => rc.borrow().next.clone(),
                None => return false,
            },
            None => return false,
        };
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_cycle() {
        let head = Some(Rc::new(RefCell::new(ListNode::new(5))));

        assert_eq!(has_cycle(head), false);

        let tail = Some(Rc::new(RefCell::new(ListNode { val: 5, next: None })));
        let head = Some(Rc::new(RefCell::new(ListNode { val: 4, next: tail })));

        assert_eq!(has_cycle(head), false);

        let tail = Some(Rc::new(RefCell::new(ListNode { val: 5, next: None })));
        let head = Some(Rc::new(RefCell::new(ListNode {
            val: 4,
            next: tail.clone(),
        })));
        tail.as_ref().unwrap().borrow_mut().next = head.clone();

        assert_eq!(has_cycle(head), true);

        let tail = Some(Rc::new(RefCell::new(ListNode { val: 5, next: None })));
        let four = Some(Rc::new(RefCell::new(ListNode {
            val: 4,
            next: tail.clone(),
        })));
        let three = Some(Rc::new(RefCell::new(ListNode {
            val: 3,
            next: four.clone(),
        })));
        let two = Some(Rc::new(RefCell::new(ListNode {
            val: 2,
            next: three.clone(),
        })));
        let head = Some(Rc::new(RefCell::new(ListNode {
            val: 1,
            next: two.clone(),
        })));
        assert_eq!(has_cycle(head.clone()), false);

        tail.as_ref().unwrap().borrow_mut().next = head.clone();

        assert_eq!(has_cycle(head), true);
    }
}
