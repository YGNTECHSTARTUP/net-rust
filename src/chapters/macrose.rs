pub fn abc() {
    macro_rules! log {
        ($exp:expr) => {
            println!("Logging Data:{}", $exp);
        };
    }
    log!("Pikachu Logged in")
}
