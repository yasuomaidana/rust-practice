use std::fmt;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub(crate) fn new_from_vec(vec: Vec<i32>) -> ListNode {
        let mut head: ListNode = ListNode::new(vec[0]);
        let mut current = &mut head;
        for item in vec.iter().skip(1) {
            current.next = Some(Box::new(ListNode::new(*item)));
            current = current.next.as_mut().unwrap();
        }
        head
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = Some(self);
        while let Some(node) = current {
            write!(f, "{}", node.val)?;
            if node.next.is_some() {
                write!(f, " -> ")?;
            }
            current = node.next.as_deref();
        }
        Ok(())
    }
}
