use std::cell::Cell;

struct Point {
    x: i32,
    y: Cell<i32>,
}

struct Inches(i32);

fn main() {
    let point = Point { x: 5, y: Cell::new(6) };

    point.y.set(7);

    println!("y : {:?}", point.y);  

    let length = Inches(10);

    let Inches(integer_lenght) = length;
    println!("length is {}f inches", integer_lenght);
}