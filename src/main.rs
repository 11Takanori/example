fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);
    println!("{}", _y);
}