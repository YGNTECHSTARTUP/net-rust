pub fn borrowing() {
    let mut s = String::from("Hello Babe");
    let h = &mut s;
    h.push_str("Pika Pika");
    let y = &mut s;
    print!("{y}")
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
