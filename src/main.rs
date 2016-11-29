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

fn main() {
    let point = Point{ x: 1.2, y: 3.4 };
    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}