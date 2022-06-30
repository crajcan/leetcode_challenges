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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(node), None) => Some(Box::new(ListNode {
            val: node.val,
            next: merge_two_lists(node.next, None),
        })),
        (None, Some(node)) => Some(Box::new(ListNode {
            val: node.val,
            next: merge_two_lists(None, node.next),
        })),
        (Some(node), Some(other_node)) => {
            if node.val < other_node.val {
                Some(Box::new(ListNode {
                    val: node.val,
                    next: merge_two_lists(node.next, Some(other_node)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: other_node.val,
                    next: merge_two_lists(Some(node), other_node.next),
                }))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(
            merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 4, next: None })),
                            })),
                        })),
                    })),
                })),
            }))
        );
    }
}
