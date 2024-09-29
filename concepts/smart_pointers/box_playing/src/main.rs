use std::fmt::Debug;

struct LinkedListNode<T: Debug>{
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

impl <T:Debug> LinkedListNode<T> {
    fn print(&self) {
        if let Some(ref next) = self.next {
            next.print();
            print!(" <- ");
        }
        print!("{:?}", self.value);
    }
}



fn main() {
    let node0 = LinkedListNode {
        value: 0,
        next: None,
    };

    let node1 = LinkedListNode {
        value: 1,
        next: Some(Box::new(node0)),
    };

    node1.print();

}
