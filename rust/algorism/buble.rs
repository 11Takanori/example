fn main() {
    let mut v = vec![5, 2, 3, 1, 4];
    let mut i = 0;
    let mut swap_flg = false;
    let limit = v.len() - 1;

    while i < limit {
        for j in 0..v.len()-i-1 {
            if v[j] > v[j+1] {
                v.swap(j, j+1);
                swap_flg = true;
            }
        }
        if swap_flg == false { break; }
        i += 1;
    }

    println!("{:?}", v);
}