struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn pop(&mut self) -> Option<T> {
        // let old_head = std::mem::replace(&mut self.head, None);
        // let old_head = std::mem::take(&mut self.head);
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }

    pub fn push(&mut self, value: T) {
        // let old_head = std::mem::replace(&mut self.head, None);
        // let old_head = std::mem::take(&mut self.head);
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            value,
            next: old_head,
        });
        self.head = Some(new_node);
    }
}

fn main() {
    println!("Hello, world!");
}
