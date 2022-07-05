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

pub fn list_length(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut count = 0;
    while let Some(n) = head {
        count += 1;
        head = &n.next;
    }
    count
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    use std::cmp::Ordering;
    let len = list_length(&head);

    match len.cmp(&n) {
        Ordering::Less => head,
        Ordering::Equal => {
            if let Some(node) = head {
                node.next
            } else {
                None
            }
        }
        Ordering::Greater => {
            let mut head = head.unwrap();
            let mut left = head.as_mut();

            for _i in 0..len - n - 1 {
                left = left.next.as_mut().unwrap();
            }

            let right = left.next.take().unwrap().next;
            left.next = right;

            Some(head)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_nth_from_end_removes_only_value() {
        let head = Some(Box::new(ListNode::new(1)));

        assert_eq!(remove_nth_from_end(head, 1), None);
    }

    #[test]
    fn test_remove_nth_from_end_leaves_only_value() {
        let head = Some(Box::new(ListNode::new(1)));
        let other_head = Some(Box::new(ListNode::new(1)));

        assert_eq!(remove_nth_from_end(head, 2), other_head);
    }

    #[test]
    fn test_remove_nth_from_end_removes_none_of_two() {
        let two = Some(Box::new(ListNode::new(2)));
        let head = Some(Box::new(ListNode { val: 1, next: two }));

        let other_two = Some(Box::new(ListNode::new(2)));
        let other_head = Some(Box::new(ListNode {
            val: 1,
            next: other_two,
        }));

        assert_eq!(remove_nth_from_end(head, 3), other_head);
    }

    #[test]
    fn test_remove_nth_from_end_removes_first_of_two() {
        let two = Some(Box::new(ListNode::new(2)));
        let head = Some(Box::new(ListNode { val: 1, next: two }));

        let other_head = Some(Box::new(ListNode::new(2)));

        assert_eq!(remove_nth_from_end(head, 2), other_head);
    }

    #[test]
    fn test_remove_nth_from_end_removes_second_of_two() {
        let two = Some(Box::new(ListNode::new(2)));
        let head = Some(Box::new(ListNode { val: 1, next: two }));

        let other_head = Some(Box::new(ListNode { val: 1, next: None }));

        assert_eq!(remove_nth_from_end(head, 1), other_head);
    }

    #[test]
    fn test_remove_nth_from_end_removes_middle_value() {
        let five = ListNode { val: 5, next: None };
        let four = ListNode {
            val: 4,
            next: Some(Box::new(five)),
        };
        let three = ListNode {
            val: 3,
            next: Some(Box::new(four)),
        };
        let two = ListNode {
            val: 2,
            next: Some(Box::new(three)),
        };
        let one = ListNode {
            val: 1,
            next: Some(Box::new(two)),
        };

        let other_five = ListNode { val: 5, next: None };
        let other_three = ListNode {
            val: 3,
            next: Some(Box::new(other_five)),
        };
        let other_two = ListNode {
            val: 2,
            next: Some(Box::new(other_three)),
        };
        let other_one = ListNode {
            val: 1,
            next: Some(Box::new(other_two)),
        };

        assert_eq!(
            remove_nth_from_end(Some(Box::new(one)), 2),
            Some(Box::new(other_one))
        );
    }
}
