struct MyLinkedList {
    data: Option<i32>,
    next: Option<Box<MyLinkedList>>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            data: None,
            next: None,
        }
    }

    fn push_front(&mut self, data: i32) {
        let mut new_node = MyLinkedList::new();
        new_node.data = Some(data);
        new_node.next = self.next.take();

        self.next = Some(Box::new(new_node));
    }

    fn push_back(&mut self, data: i32) {
        let mut indexer = self;
        while let Some(ref mut next) = indexer.next {
            indexer = next;
        }

        let mut new_node = MyLinkedList::new();
        new_node.data = Some(data);
        new_node.next = None;

        indexer.next = Some(Box::new(new_node));
    }

    fn count(&self) -> u32 {
        let mut count: u32 = 0;
        let mut indexer = self;

        while let Some(ref next) = indexer.next {
            count += 1;
            indexer = next;
        }

        count
    }

    fn print(&self) {
        let mut indexer = self;
        let mut index: u32 = 0;
        while let Some(ref next) = indexer.next {
            match indexer.data {
                Some(value) => {
                    println!("Node {}: {}", index, value);
                    index += 1;
                }
                None => println!("HEAD"),
            }
            indexer = next;
        }
        println!("Node {}: {}", index, indexer.data.unwrap_or_default());
        println!("Count: {}", self.count());
    }
}

fn main() {
    let mut ll = MyLinkedList::new();
    ll.push_front(10);
    ll.push_back(30);
    ll.push_back(40);

    ll.print();
}
