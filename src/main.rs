fn analyze_slice(slice: &[i32]) {
    println!("{}", slice[0]);
    println!("{}", slice[1]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let tuple = (1, 2, 3);

    let y = tuple.1;

    println!("first element of the array: {}", xs[0]);

    analyze_slice(&xs[2 .. 4]);

    for x in 0..10 {
        println!("{}", x);
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    println!("y is {}", y);

    let mut x = 5; 
    let mut done = false; 

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}
