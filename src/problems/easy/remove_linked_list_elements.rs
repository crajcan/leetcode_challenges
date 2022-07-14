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

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut current_node = head.as_mut().unwrap();

    while let Some(next_node) = current_node.next.as_mut() {
        if next_node.val == val {
            current_node.next = next_node.next.take();
        } else {
            current_node = current_node.next.as_mut().unwrap();
        }
    }

    if head.as_ref().unwrap().val == val {
        return head.unwrap().next;
    }

    head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_elements() {
        let head = Some(Box::new(ListNode::new(5)));
        let head = Some(Box::new(ListNode { val: 4, next: head }));
        let head = Some(Box::new(ListNode { val: 3, next: head }));
        let head = Some(Box::new(ListNode { val: 2, next: head }));
        let head = Some(Box::new(ListNode { val: 1, next: head }));

        let exp = Some(Box::new(ListNode::new(5)));
        let exp = Some(Box::new(ListNode { val: 4, next: exp }));
        let exp = Some(Box::new(ListNode { val: 2, next: exp }));
        let exp = Some(Box::new(ListNode { val: 1, next: exp }));

        assert_eq!(remove_elements(head, 3), exp);

        let head = Some(Box::new(ListNode::new(5)));
        let head = Some(Box::new(ListNode { val: 4, next: head }));

        let exp = Some(Box::new(ListNode::new(5)));

        assert_eq!(remove_elements(head, 4), exp);
    }
}
