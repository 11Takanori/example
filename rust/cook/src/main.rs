extern crate crossbeam;

use std::cmp;

fn find_max(arr: &[i32], start: usize, end: usize) -> i32 {
    const THRESHOLD: usize = 2;
    if end - start <= THRESHOLD {
        return *arr.iter().max().unwrap();
    }

    let mid = start + (end - start) / 2;
    crossbeam::scope(|scope| {
        let left = scope.spawn(|| find_max(arr, start, mid));
        let right = scope.spawn(|| find_max(arr, mid, end));

        cmp::max(left.join(), right.join())
    })
}

fn main() {
    let arr = &[-4, 1, 10, 25];
    let max = find_max(arr, 0, arr.len());

    assert_eq!(25, max);
}