#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32, next: Option<Box<Node>>) -> Node {
        Node{value: value, next: next}
    }
}

#[derive(Debug)]
pub struct LinkedIntList {
    first: Option<Box<Node>>,
}


impl LinkedIntList {
    pub fn new() -> LinkedIntList {
        LinkedIntList{first: None}
    }

    pub fn push(&mut self, value: i32){
        let old_first = self.first.take();
        self.first = Some(Box::new(Node::new(value, old_first)));
    }
}
