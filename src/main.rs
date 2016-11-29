use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
    println!("Now {} will print!", Structure(3));
    println!("Now {} will print!", Deep(Structure(7)));
}