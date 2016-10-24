fn main() {
    let mut m = 5;

    {
        let mut add_n = |x: i32| m += x;
        add_n(5);
    }

    assert_eq!(10, m);
}