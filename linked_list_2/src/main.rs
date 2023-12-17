struct QNode {
    data: Option<i32>,
    next: Option<Box<QNode>>,
}

struct QLinkedList {
    head: Option<QNode>,
    size: u32,
}

impl QLinkedList {
    fn new() -> Self {
        QLinkedList {
            head: Some(QNode {
                data: None,
                next: None,
            }),
            size: 0,
        }
    }

    fn push_front(&mut self, data: i32) -> &mut QLinkedList {
        let first_node = self.head.as_mut().and_then(|head| head.next.take());
        let new_node = QNode {
            data: Some(data),
            next: first_node,
        };
        self.head.as_mut().unwrap().next = Some(Box::new(new_node));
        self.size += 1;
        self
    }

    fn pop_front(&mut self) -> &mut QLinkedList {
        let first_node = self.head.as_mut().and_then(|head| head.next.take());
        match first_node {
            Some(node) => {
                self.head.as_mut().unwrap().next = node.next;
                self.size -= 1;
            }
            None => {
                self.head.as_mut().unwrap().next = None;
                self.size = 0;
            }
        }
        self
    }

    fn push_back(&mut self, data: i32) -> &mut QLinkedList {
        unimplemented!()
    }

    fn pop_back(&mut self) -> &mut QLinkedList {
        unimplemented!()
    }

    fn print(&self) {
        let mut indexer = &self.head.as_ref().unwrap().next;
        let mut index: u32 = 0;
        while let Some(node) = indexer {
            if let Some(data) = &node.data {
                println!("Node {}: {}", index, data);
            } else {
                panic!("Something went wrong!!!");
            }
            index += 1;
            indexer = &node.next;
        }
        println!("Size: {}", self.size);
        println!("---------------------------");
    }
}

fn main() {
    let mut ll = QLinkedList::new();

    ll.push_front(10).push_front(5).push_front(0);
    ll.print();

    ll.pop_front();
    ll.print();

    ll.pop_front();
    ll.print();

    ll.pop_front();
    ll.print();
}
