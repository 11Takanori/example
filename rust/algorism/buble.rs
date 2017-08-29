fn main() {
    let mut v   = vec![2, 1, 3, 5, 4];
    let mut len = v.len();

    while len > 0 {
        for i in 0..v.len()-1 {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
            }
        }
        len -= 1;
    }
    println!("{:?}", v);
}