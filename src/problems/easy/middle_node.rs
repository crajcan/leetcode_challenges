//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut temp = head.as_ref();
    let mut length = 0;

    while let Some(node) = temp {
        length += 1;
        temp = node.next.as_ref();
    }

    for _i in 0..length / 2 {
        head = head.unwrap().next;
    }

    head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_middle_node() {
        // list with [1,2,3,4,5,6] in it
        let six = ListNode::new(6);
        let five = ListNode {
            val: 5,
            next: Some(Box::new(six)),
        };
        let four = ListNode {
            val: 4,
            next: Some(Box::new(five)),
        };
        let other_four = four.clone();

        let three = ListNode {
            val: 3,
            next: Some(Box::new(four)),
        };
        let two = ListNode {
            val: 2,
            next: Some(Box::new(three)),
        };
        let head = ListNode {
            val: 1,
            next: Some(Box::new(two)),
        };

        assert_eq!(
            middle_node(Some(Box::new(head))),
            Some(Box::new(other_four))
        );
    }
}
