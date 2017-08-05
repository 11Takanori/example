use std::io::Write;
use std::io;

fn main() {
    loop {
        print!(">> ");
        if let Err(e) = io::stdout().flush() { panic!(e); }
        let mut line = "".to_string();
        if let Err(_) = io::stdin().read_line(&mut line) { break; }
        if line.is_empty() { break; }
    }
}
