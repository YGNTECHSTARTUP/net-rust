pub fn calculate<F>(f: F) -> i32
where
    F: FnMut(i32) -> i32,
{
    (0..5).map(f).sum()
}
