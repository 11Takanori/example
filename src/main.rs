use std::fmt;
use std::fmt::{Formatter, Display};
use List2::*;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

#[derive(Debug)]
struct List(Vec<i32>);

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[derive(Debug)]
enum List2 {
    Cons(u32, Box<List2>),
    Nil,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Color2 {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2 :Point,
}

struct Pair(Box<i32>, Box<i32>);

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1- y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag < 0. {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        write!(f, "]")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
               self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

impl List2 {
    fn new() -> List2 {
        Nil
    }

    fn prepend(self, elem: u32) -> List2 {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn transpose(pair: Matrix) -> Matrix {
    let Matrix(t1, t2, t3, t4) = pair;

    Matrix(t1, t3, t2, t4)
}

fn age() -> u32 {
    15
}

fn main() {
    let p = Point{ x: 1.2, y: 3.4 };
    let c = Complex { real: 3.3, imag: 7.2 };
    let v = List(vec![1, 2, 3]);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    let mut list = List2::new();
    let color = Color2::RGB(122, 17, 40);

    println!("Display: {}", p);
    println!("Debug: {:?}", p);

    println!("Display: {}", c);
    println!("Debug: {:?}", c);

    println!("Display: {}", v);
    println!("Debug: {:?}", v);

    for color in [
        Color { red: 128, green: 255, blue: 90},
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}",  list.stringify());

    match color {
        Color2::Red   => println!("The color is Red!"),
        Color2::Blue  => println!("The color is Blue!"),
        Color2::Green => println!("The color is Green!"),
        Color2::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color2::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color2::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color2::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yello: {}!", c, m, y),
        Color2::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yello: {}, key (black): {}!",
                c, m, y, k),
    }

    let reference = &4;
    let value = 5;
    let mut mut_value = 6;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    match value {
        ref f => println!("Got a reference to a value: {:?}", f),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    match age() {
        0             => println!("I'm not born yet I guess"),
        n @ 1 ... 12  => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater tham 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}