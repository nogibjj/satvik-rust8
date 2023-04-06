struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node));
    }

}

fn main() {
    let n0 = Node { value: 0, next: None };
    let mut ll = LinkedList{head: Some(Box::new(n0))};
    ll.push_front(42)
}