fn change_truth(x: bool) -> bool {
    !x
}

fn main() {
    let a = true;

    let _y = change_truth(a);
    println!("{}", a);
    println!("{}", _y);
}