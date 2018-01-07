use std::io::{self, BufRead, Write};
use std::process;

#[derive(Debug)]
struct InputBuffer {
    buffer: String,
    buffer_length: usize,
    input_length: usize,
}

impl InputBuffer {
    fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            buffer_length: 0,
            input_length: 0,
        }
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("db > ");
        io::stdout().flush().expect("Error flushing stdout");

        let input_buffer = InputBuffer::new();
        let mut line = input_buffer.buffer;
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        match line.trim() {
            ".exit" => process::exit(0x0100),
            _ => println!("Unrecognized command {}.", line.trim()),
        }
    }
}
