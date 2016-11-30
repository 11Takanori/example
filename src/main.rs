use std::fmt;

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

fn main() {
    let p = Point{ x: 1.2, y: 3.4 };
    let c = Complex { real: 3.3, imag: 7.2 };
    let v = List(vec![1, 2, 3]);

    println!("Display: {}", p);
    println!("Debug: {:?}", p);

    println!("Display: {}", c);
    println!("Debug: {:?}", c);

    println!("Display: {}", v);
    println!("Debug: {:?}", v);
}