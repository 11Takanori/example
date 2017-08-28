#[derive(Debug)]
struct Member {
    no: i32,
    name: String,
}

impl Member {
    fn new() -> Member  {
        Member {
            no: 0,
            name: "".to_string(),
        }
    }
}

enum Node<T> {
    Member,
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

#[derive(Debug)]
struct Node {
    data: Member,
    next: Box<Node>,
}

impl Node {
     fn new() -> Node {
         Node {
             data: Member::new(),
             next: Box<Node::new()>,
         }
     }
}

#[derive(Debug)]
struct ChainHash {
    size: i32,
    table: Node,
}


fn main() {
    let member = Member::new();
    println!("{}, {}", member.no, member.name);
}