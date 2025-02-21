fn example_1() {
    let r;
    {
        let x = 42;
        r = &x;
        println!("{r}")
    }
}
