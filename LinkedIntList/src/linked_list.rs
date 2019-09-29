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

    pub fn pop(&mut self) -> i32 {
        let Node{value: node_value, next: node_next} = *(self.first.unwrap());
        self.first = node_next;
        node_value

    }
    //
    // pub fn append(&mut self, value: i32) {
    //     let mut end = *self.first.take().unwrap();
    //     let mut temp =
    //
    //     while !end.next.is_none() {
    // //         end = *end.next.unwrap();
    // //     }
    //
    //     //add a new node to the end of the linked linked list
    //     end.next = Some(Box::new(Node::new(value, None)));
    // }



}
