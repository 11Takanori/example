use List::*;

#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
     fn new() -> List {
         Nil
     }

    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }
}

#[derive(Debug)]
struct HashTable {
    bins: List
}

impl HashTable {
     fn new(list: List) -> HashTable {
         HashTable {
             bins: list
         }
     }
}

fn main() {
    let mut list = List::new();

    // list = list.prepend(1);
    // list = list.prepend(2);
    // list = list.prepend(3);
    // list = list.prepend(4);

    let hash_table = HashTable::new(list);

    println!("{:?}", hash_table);

}