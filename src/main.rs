
fn main() {
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print...", Structure(3));
    println!("My name is {0}, {1} {0}", "James","Bond");
}