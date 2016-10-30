extern crate introduction;

use introduction::english::*;
use introduction::japanese;
use std::mem;
use std::fmt;

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
}

trait Graph {
    type N: fmt::Display;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}
