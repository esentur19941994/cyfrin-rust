use std::cmp::PartialOrd;

// Exercise 1
pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

// Exercise 2
pub fn zip<T: Copy, U: Copy>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    let mut v = vec![];
    let len = min(a.len(), b.len()); 

    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}