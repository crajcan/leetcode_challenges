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

pub fn delete_helper(head: Option<Box<ListNode>>, previous: Option<i32>) -> Option<Box<ListNode>> {
    match head {
        Some(b) => {
            let ListNode { next, val } = *b;

            match previous {
                None => Some(Box::new(ListNode {
                    val,
                    next: delete_helper(next, Some(val)),
                })),
                Some(v) if v == val => delete_helper(next, previous),
                _ => Some(Box::new(ListNode {
                    val,
                    next: delete_helper(next, Some(val)),
                })),
            }
        }
        None => None,
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    delete_helper(head, None)
}
pub fn delete_duplicates_iterative(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut current_node = head.as_mut().unwrap();

    while let Some(next_node) = current_node.next.as_mut() {
        if current_node.val == next_node.val {
            current_node.next = next_node.next.take()
        } else {
            current_node = current_node.next.as_mut().unwrap()
        }
    }
    head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let head = Some(Box::new(ListNode::new(5)));
        let head = Some(Box::new(ListNode { val: 5, next: head }));
        let head = Some(Box::new(ListNode { val: 4, next: head }));
        let head = Some(Box::new(ListNode { val: 4, next: head }));
        let head = Some(Box::new(ListNode { val: 3, next: head }));
        let head = Some(Box::new(ListNode { val: 2, next: head }));
        let head = Some(Box::new(ListNode { val: 2, next: head }));
        let head = Some(Box::new(ListNode { val: 1, next: head }));
        let head = Some(Box::new(ListNode { val: 1, next: head }));

        let expected = Some(Box::new(ListNode::new(5)));
        let expected = Some(Box::new(ListNode {
            val: 4,
            next: expected,
        }));
        let expected = Some(Box::new(ListNode {
            val: 3,
            next: expected,
        }));
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: expected,
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: expected,
        }));

        assert_eq!(delete_duplicates(head), expected);
    }

    #[test]
    fn test_delete_duplicates_iterative() {
        let head = Some(Box::new(ListNode::new(5)));
        let head = Some(Box::new(ListNode { val: 5, next: head }));
        let head = Some(Box::new(ListNode { val: 4, next: head }));
        let head = Some(Box::new(ListNode { val: 4, next: head }));
        let head = Some(Box::new(ListNode { val: 3, next: head }));
        let head = Some(Box::new(ListNode { val: 2, next: head }));
        let head = Some(Box::new(ListNode { val: 2, next: head }));
        let head = Some(Box::new(ListNode { val: 1, next: head }));
        let head = Some(Box::new(ListNode { val: 1, next: head }));

        let expected = Some(Box::new(ListNode::new(5)));
        let expected = Some(Box::new(ListNode {
            val: 4,
            next: expected,
        }));
        let expected = Some(Box::new(ListNode {
            val: 3,
            next: expected,
        }));
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: expected,
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: expected,
        }));

        assert_eq!(delete_duplicates_iterative(head), expected);
    }
}
