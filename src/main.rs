extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

fn main() {
    println!("{:?}", json::encode(&42));
}