fn analyze_slice(slice: &[i32]) {
    println!("{}", slice[0]);
    println!("{}", slice[1]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    println!("first element of the array: {}", xs[0]);

    analyze_slice(&xs[2 .. 4]);

 

    for x in 0..10 {
        println!("{}", x);
    }

}
