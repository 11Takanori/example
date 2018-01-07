use std::io::{self, BufRead, Write};
use std::process;

#[derive(Debug)]
struct InputBuffer {
    buffer: char,
    buffer_length: usize,
    input_length: usize,
}

impl InputBuffer {
     fn new() -> InputBuffer {
         unimplemented!();
     }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("db > ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        match line.trim() {
            ".exit" => process::exit(0x0100),
            _ => println!("Unrecognized command {}.", line.trim()),
        }
    }
}