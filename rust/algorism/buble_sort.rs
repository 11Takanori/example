fn main() {
    let mut v = vec![5, 2, 3, 1, 4];
    let mut i = 0;
    let limit = v.len() - 1;
    let mut last = limit;

    while i < limit {
        for j in 0..last {
            if v[j] > v[j+1] {
                v.swap(j, j+1);
                last = j;
            }
        }
        i += 1;
    }

    println!("{:?}", v);
}