use std::{iter::Sum, ops::Add};

pub fn borrowing() {
    let mut s = String::from("Hello Babe");
    let h = &mut s;
    h.push_str("Pika Pika");
    let y = &mut s;
    print!("{y}")
}

pub fn generics() {
    let list = vec![1, 2, 3, 4, 5];
    let list1 = vec![1.2, 32.23, 32.3, 32.3];
    println!("{}", long(list));
    println!("{}", long(list1));
}

fn long<T>(x: Vec<T>) -> T
where
    T: Add<Output = T> + Default,
{
    let mut sum = T::default();
    for i in x {
        sum = sum + i;
    }
    sum
}

// pub fn lifetime() {
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = vec![1, 2];
//     println!("{:?}", longer_vector(&v1, &v2));
// }

// fn longer_vector(x: &[i32], y: &[i32]) -> &[i32] {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
