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

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut res = 0;
    let mut next = &head;

    while let Some(node) = next {
        res *= 2;

        match node.val {
            0 => (),
            _ => res += 1,
        }

        next = &node.next
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_decimal_value() {
        assert_eq!(get_decimal_value(Some(Box::new(ListNode::new(0)))), 0);
        assert_eq!(get_decimal_value(Some(Box::new(ListNode::new(1)))), 1);

        let next_next = Some(Box::new(ListNode { next: None, val: 1 }));
        let next = Some(Box::new(ListNode {
            next: next_next,
            val: 0,
        }));
        let root = Some(Box::new(ListNode { next, val: 1 }));

        assert_eq!(get_decimal_value(root), 5);
    }
}
