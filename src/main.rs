fn main() {
    let mut a: String = "abc".to_string();
    {
        let b: &mut String = &mut a;
        b.push_str("xyz");

        println!("b={}", b);
    }
    println!("a={}", a);

    {
        let b: &mut String = &mut a;
        let c: &mut String = b;

        println!("c={}", c);
    }

    {
        let b: &String = &a;
        println!("a={}, b={}", a, b);

        let c: &String = &a;
        println!("a={}, b={}, c={}", a, b, c);
    }
}