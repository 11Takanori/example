use List::*;

#[derive(Clone, Debug)]
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

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

const  BIN_SIZE: usize = 4;

#[derive(Debug)]
struct HashTable {
    bins: Vec<List>
}

impl HashTable {
     fn new() -> HashTable {
         HashTable {
             bins: vec![List::new(); BIN_SIZE]
         }
     }

     fn set(&mut self, key: u32, value: String) {
         let index = key as usize % BIN_SIZE;

         for bin in &self.bins {
            match *bin {
                Cons(head, ref tail) => {
                },
                Nil => {
                },
            }
         }
         self.bins[index] = List::new().prepend(index as u32);
     }
}

fn main() {
    let mut hash_table = HashTable::new();
    hash_table.set(6, "a".to_string());

    println!("{:?}", hash_table);

}