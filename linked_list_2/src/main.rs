#![allow(dead_code)]

struct MyNode {
    data: Option<i32>,
    next: Option<Box<MyNode>>,
}

struct MyLinkedList {
    head: Option<MyNode>,
    size: u32,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: Some(MyNode {
                data: None,
                next: None,
            }),
            size: 0,
        }
    }

    fn push_front(&mut self, data: i32) -> &mut MyLinkedList {
        let first_node = self.head.as_mut().and_then(|head| head.next.take());
        let new_node = MyNode {
            data: Some(data),
            next: first_node,
        };
        self.head.as_mut().unwrap().next = Some(Box::new(new_node));
        self.size += 1;
        self
    }

    fn pop_front(&mut self) -> &mut MyLinkedList {
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

    fn push_back(&mut self, data: i32) -> &mut MyLinkedList {
        let mut current_node = &mut self.head.as_mut().unwrap().next;
        while let Some(node) = current_node {
            current_node = &mut node.next;
        }

        let new_node = MyNode {
            data: Some(data),
            next: None,
        };
        *current_node = Some(Box::new(new_node));
        self.size += 1;
        self
    }

    fn pop_back(&mut self) -> &mut MyLinkedList {
        unimplemented!("I gave up. Fuck this language")
    }

    fn print(&self) {
        let mut indexer = &self.head.as_ref().unwrap().next;
        let mut index: u32 = 0;
        println!("---------------------------");
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
    let mut ll = MyLinkedList::new();
    ll.push_front(2)
        .push_front(1)
        .push_front(0)
        .push_back(3)
        .push_back(4)
        .push_back(5);

    ll.print();
}
