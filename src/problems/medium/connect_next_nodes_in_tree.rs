// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn connect(root: &mut Option<Rc<RefCell<Node>>>) {
    match root {
        None => (),
        Some(rc) => {
            let mut root = rc.borrow_mut();

            match &root.left {
                None => (),
                Some(left) => {
                    // point left at right
                    left.borrow_mut().next = root.right.clone();

                    // if next exists, point right at next.left
                    match &root.next {
                        None => (),
                        Some(next) => {
                            root.right.as_ref().unwrap().borrow_mut().next =
                                next.borrow().left.clone();
                        }
                    }

                    connect(&mut root.left);
                    connect(&mut root.right);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_connect() {
        let mut one = Node::new(1);
        let mut two = Node::new(2);
        let mut three = Node::new(3);
        let four = Node::new(4);
        let five = Node::new(5);
        let six = Node::new(6);
        let seven = Node::new(7);

        two.left = Some(Rc::new(RefCell::new(four)));
        two.right = Some(Rc::new(RefCell::new(five)));
        three.left = Some(Rc::new(RefCell::new(six)));
        three.right = Some(Rc::new(RefCell::new(seven)));
        one.left = Some(Rc::new(RefCell::new(two)));
        one.right = Some(Rc::new(RefCell::new(three)));

        let mut other_one = Node::new(1);
        let mut other_two = Node::new(2);
        let mut other_three = Node::new(3);
        let mut other_four = Node::new(4);
        let mut other_five = Node::new(5);
        let mut other_six = Node::new(6);
        let other_seven = Node::new(7);

        other_three.right = Some(Rc::new(RefCell::new(other_seven)));
        other_six.next = other_three.right.clone();
        other_three.left = Some(Rc::new(RefCell::new(other_six)));
        other_five.next = other_three.left.clone();

        other_two.right = Some(Rc::new(RefCell::new(other_five)));
        other_four.next = other_two.right.clone();
        other_two.left = Some(Rc::new(RefCell::new(other_four)));

        other_one.right = Some(Rc::new(RefCell::new(other_three)));
        other_two.next = other_one.right.clone();
        other_one.left = Some(Rc::new(RefCell::new(other_two)));

        assert_ne!(one, other_one);
        let mut root = Some(Rc::new(RefCell::new(one)));
        // println!("{:?}", root);
        connect(&mut root);
        // println!("one: {:#?}", one);
        // println!("other_one: {:#?}", other_one);

        assert_eq!(root, Some(Rc::new(RefCell::new(other_one))));
    }
}
