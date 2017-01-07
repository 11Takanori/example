#[macro_use]
extern crate itertools;

use itertools::Itertools;

fn main() {
    let mut words = "hello supercalifragilisticexpialidocious programmer".split(|c| c == ' ');
    words.foreach(|word| println!("{} is {} characters long", word, word.len()));
}