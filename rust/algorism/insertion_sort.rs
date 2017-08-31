fn main() {
    let mut v = vec![5, 2, 3, 1, 4];

    for i in 1..v.len() {
        let tmp = v[i];
        let mut j = i;
        while j > 0 && v[j-1] > tmp {
            v[j] = v[j-1];
            j -= 1;
        }
        v[j] = tmp;
    }

    println!("{:?}", v);
}
