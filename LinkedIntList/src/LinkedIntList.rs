
struct Node {
    value: i32,
    next: Option<Node>,
}

struct LinkedIntList {
    first: Option<Node>,
}


impl LinkedIntList {
    pub fn new() -> LinkedIntList {
        LinkedIntList{first: None}
    }

    pub fn push(&mut self, value: i32){
        let first = self.first;
        self.first = Node{value: value, next: Some(first)};
    }
}
