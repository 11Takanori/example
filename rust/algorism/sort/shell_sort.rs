fn main() {
    let mut v = vec![5, 2, 3, 1, 4];
    let mut h = v.len() / 2;

    while h > 0 {
        for i in h..v.len() {
            let mut j = i;
            let tmp = v[j];
            while h <= j && v[j - h] > tmp {
                v[j] = v[j - h];
                j -= h;
            }
            v[j] = tmp;
        }
        h /= 2;
    }

    println!("{:?}", v);
}
