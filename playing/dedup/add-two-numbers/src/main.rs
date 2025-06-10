use crate::list_node::ListNode;

mod list_node;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut result_node = ListNode::new(0);
    let mut work_node = &mut result_node;
    let mut wl1 = l1;
    let mut wl2 = l2;
    while wl1.is_some() || wl2.is_some() || carry > 0 {
        let val1 = wl1.as_ref().map_or(0, |n| n.val);
        let val2 = wl2.as_ref().map_or(0, |n| n.val);
        let sum = val1 + val2 + carry;
        carry = sum / 10;
        work_node.val = sum % 10;

        if wl1.is_some() {
            wl1 = wl1.and_then(|n| n.next);
        }
        if wl2.is_some() {
            wl2 = wl2.and_then(|n| n.next);
        }

        if wl1.is_some() || wl2.is_some() || carry > 0 {
            work_node.next = Some(Box::new(ListNode::new(0)));
            work_node = work_node.next.as_mut().unwrap();
        } else {
            work_node.next = None; // Ensure the last node does not point to a new node
        }
    }
    Some(Box::new(result_node))
}

fn main() {
    let l1 = ListNode::new_from_vec(vec![2, 4, 3]);
    println!("l1: {}", l1);
    let l2 = ListNode::new_from_vec(vec![5, 6, 4]);
    println!("l2: {}", l2);
    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    if let Some(res) = result {
        println!("Result: {}", res);
    } else {
        println!("Result is None");
    }

    let l1 = ListNode::new_from_vec([9, 9, 9, 9, 9, 9, 9].to_vec());
    let l2 = ListNode::new_from_vec(vec![9, 9, 9, 9].to_vec());
    println!("l1: {}", l1);
    println!("l2: {}", l2);
    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    if let Some(res) = result {
        println!("Result: {}", res);
    } else {
        println!("Result is None");
    }
}
