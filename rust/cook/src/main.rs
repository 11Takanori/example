use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::error::Error;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<Error>> {
    let path = "lines.txt";
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}



