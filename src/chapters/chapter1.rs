pub fn borrowing() {
    let mut s = String::from("Hello Babe");
    let h = &mut s;
    h.push_str("Pika Pika");
    let y = &mut s;
    print!("{y}")
}
