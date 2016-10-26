#[derive(Debug)]
struct Foo<'a> {
    x: &'a i32
}

fn main() {
    let y = &5;
    let f = Foo { x: y };

    println!("{}", f.x);
}