// Definition for singly-linked list.
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

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = None;

    while let Some(mut node) = head {
        let next = node.next;

        node.next = new_head;

        new_head = Some(node);

        head = next;
    }

    new_head
}

pub fn reverse_helper(
    new_head: Option<Box<ListNode>>,
    mut head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match head {
        None => new_head,
        Some(ref mut node) => {
            let next = node.next.take();

            node.next = new_head;

            reverse_helper(head, next)
        }
    }
}

pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    reverse_helper(None, head)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let five = ListNode::new(5);
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

        let other_one = ListNode::new(1);
        let other_two = ListNode {
            val: 2,
            next: Some(Box::new(other_one)),
        };
        let other_three = ListNode {
            val: 3,
            next: Some(Box::new(other_two)),
        };
        let other_four = ListNode {
            val: 4,
            next: Some(Box::new(other_three)),
        };
        let other_five = ListNode {
            val: 5,
            next: Some(Box::new(other_four)),
        };

        assert_eq!(
            reverse_list(Some(Box::new(one))),
            Some(Box::new(other_five))
        );
    }

    #[test]
    fn test_reverse_list_recursive() {
        let five = ListNode::new(5);
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

        let other_one = ListNode::new(1);
        let other_two = ListNode {
            val: 2,
            next: Some(Box::new(other_one)),
        };
        let other_three = ListNode {
            val: 3,
            next: Some(Box::new(other_two)),
        };
        let other_four = ListNode {
            val: 4,
            next: Some(Box::new(other_three)),
        };
        let other_five = ListNode {
            val: 5,
            next: Some(Box::new(other_four)),
        };

        assert_eq!(
            reverse_list_recursive(Some(Box::new(one))),
            Some(Box::new(other_five))
        );
    }
}
