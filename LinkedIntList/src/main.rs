mod linked_list;

use linked_list::LinkedIntList;

fn main() {
    let mut l1 = LinkedIntList::new();
    l1.push(6);
    println!("{:?}", l1);
    l1.push(5);
    println!("{:?}", l1);

    println!("Starting push");

    //l1.append(7);

    println!("{:?}", l1);

    let first = l1.pop();
    println!("{:?}", l1);


}
