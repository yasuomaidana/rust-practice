use crate::list_node::ListNode;

mod list_node;

fn list_node_to_vec(node: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = node;
    while let Some(n) = current {
        vec.push(n.val);
        current = &n.next;
    }
    vec
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut stack1 = list_node_to_vec(&l1);
    let mut stack2 = list_node_to_vec(&l2);
    let mut result = std::collections::VecDeque::new();

    while !stack1.is_empty() || !stack2.is_empty() || carry > 0 {
        let sum = carry + stack1.pop().unwrap_or(0) + stack2.pop().unwrap_or(0);
        carry = sum / 10;
        let digit = sum % 10;

        // Push the digit onto the result stack
        result.push_back(digit);
        // result.push(digit); // Using stack1 to store the result for simplicity
    }
    let mut result_node = ListNode::new(result.pop_front().unwrap_or(0));
    let mut current = &mut result_node;
    while let Some(digit) = result.pop_front() {
        current.next = Some(Box::new(ListNode::new(digit)));
        current = current.next.as_mut().unwrap();
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
