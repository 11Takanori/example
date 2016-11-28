
fn main() {
    #[derive(Debug)]
    struct Structure(i32);

    let pi = format!("{:.*}", 3, 22 as f32 /7 as f32);

    println!("This struct `{:?}` won't print...", Structure(3));
    println!("My name is {0}, {1} {0}", "James","Bond");
    println!("Pi is roughly {:?}", pi);
}