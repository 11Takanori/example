fn main() {
    let mut v = vec![5, 2, 3, 1, 4];

    for i in 0..v.len()-1 {
        let mut min = i;
        for j in i+1..v.len(){
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }

    println!("{:?}", v);
}