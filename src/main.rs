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

fn main() {
    let p = Point{ x: 1.2, y: 3.4 };
    let c = Complex { real: 3.3, imag: 7.2 };
    let v = List(vec![1, 2, 3]);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    let mut list = List2::new();

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
}