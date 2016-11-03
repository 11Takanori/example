extern crate introduction;

use introduction::english::*;
use introduction::japanese;
use std::mem;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());

    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);

        println!("{}", b);
    }

    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    let mut string = format!("my friend");
    let r = &mut string;

    print_out(name);

    greet(r);
    greet(r);   

    println!("Hello, {}!", r);
}

fn two_words() -> (String, String) {
    let fello = format!("fello");
    let rustaceans = format!("restaceans");

    (fello, rustaceans)
}

fn greet(name: &mut String) {
    name.push_str("foo");
}

fn remove_vowels(name: String) -> (String, String) {
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
            }
            _ => {
                output.push(c);
            }
        }
    }
    (output, name)
}

fn print_out(name: String) {
    let (devowelized_name, name) = remove_vowels(name);
    println!("Removing vowels yields {:?}", devowelized_name);

    println!("Removing vowels from {:?} yields {:?}",
            name, devowelized_name);
}

trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

struct Node;
struct Edge;
struct MyGraph;

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}