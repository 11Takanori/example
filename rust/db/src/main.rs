use std::io::{self, BufRead, Write};
use std::process;

#[derive(Debug)]
enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND,
}

#[derive(Debug)]
enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNIZED_STATEMENT,
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
