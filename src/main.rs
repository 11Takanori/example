extern crate introduction;

use introduction::english::*;
use introduction::japanese;
use std::mem;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());

    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    let mut string = format!("my friend");
    print_out(name);

    {
        let r = &mut string;
        greet(r);
        greet(r);
    }

    println!("Hello, {}!", string);

    let (mut str1, str2) = two_words();
    join_words(&mut str1, &str2);
    println!("concatenated string id {}", str1);
}

fn two_words() -> (String, String) {
    let fello = format!("fellow");
    let rustaceans = format!("Rustaceans");

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

fn join_words(prefix: &mut String, suffix: &str) {
    prefix.push(' ');
    prefix.push_str(suffix);
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