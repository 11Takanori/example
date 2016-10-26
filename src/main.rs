#[derive(Debug)]
struct Foo<'a> {
    x: &'a i32,
}

impl <'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let y = &5;
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}