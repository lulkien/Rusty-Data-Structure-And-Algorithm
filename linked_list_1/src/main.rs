#[derive(Debug)]
struct MyLinkedList(Option<(i32, Box<MyLinkedList>)>);

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList(None)
    }

    fn push_front(&mut self, data: i32) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(MyLinkedList(t))));
    }

    fn push_back(&mut self, data: i32) {
        match self.0 {
            Some((_, ref mut next)) => next.push_back(data),
            None => self.push_front(data),
        }
    }

    fn count(&self) -> u32 {
        match self.0 {
            Some((_, ref next)) => 1 + next.count(),
            None => 0,
        }
    }

    fn print_index(&self, index: u32) {
        match self.0 {
            Some((data, ref next)) => {
                println!("Node {}: {}", index, data);
                next.print_index(index + 1);
            }
            None => (),
        }
    }

    fn print(&self) {
        self.print_index(0);
        println!("Count: {}", self.count());
    }
}

fn main() {
    let mut ll = MyLinkedList::new();
    ll.push_front(20);
    ll.push_back(30);
    ll.push_front(10);

    ll.print();
}
