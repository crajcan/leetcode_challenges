use crate::list_node::ListNodei32 as ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_helper(l1, l2, 0)
}

 
pub fn add_two_numbers_helper(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    let (sum, next_1, next_2) = match (l1, l2) {
        (None, None) => {
            if carry == 0 {
                return None;
            } else {
                return Some(Box::new(ListNode {next: None, val: carry}));
            }
        }
        (Some(n1), None) => {
            (n1.val + carry, n1.next, None)
        }
        (None, Some(n2)) => {
            (n2.val + carry, None, n2.next) 
        }
        (Some(n1), Some(n2)) => {
            (n1.val + n2.val + carry, n1.next, n2.next)
        },
    };

    if sum > 9 {
        let this_digit = sum % 10;
        let carry = sum / 10;

        Some(Box::new(ListNode {
            next: add_two_numbers_helper(next_1, next_2, carry),
            val: this_digit
        })) 
    } else {
        Some(Box::new(ListNode {
            next: add_two_numbers_helper(next_1, next_2, 0),
            val: sum
        })) 
    }            
}


fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v.into_iter().rev() {
        let mut node = ListNode::new(i);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: None,
                    val: 3
                })),
                val: 4
            })),
            val: 2
        }));

        let l2 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: None,
                    val: 4
                })),
                val: 6
            })),
            val: 5
        }));

        let expected = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: None,
                    val: 8
                })),
                val: 0
            })),
            val: 7
        }));

        assert_eq!(add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_add_two_large_numbers() {
        let l1 = vec![9,9,9,9,9,9,9];
        let l2 = vec![9,9,9,9];

        let l1 = vec_to_list(l1.to_vec());
        let l2 = vec_to_list(l2.to_vec());

        let expected = 
            vec_to_list(vec![8,9,9,9,0,0,0,1].to_vec());
        
        assert_eq!(add_two_numbers(l1, l2), expected);
    }
}













