use anyhow::{Context, Error};
use std::{f64::consts, fs::read_to_string};
use thiserror::{self, Error};
#[derive(Debug, Error)]
pub enum MyError {
    #[error("Network Connection Failed")]
    NetworkError,
    #[error("TimedOut")]
    TimeOutError,
}

fn errkaro() -> Result<(), MyError> {
    Err(MyError::NetworkError)
}

pub fn check() {
    let c = errkaro();
    println!("{:?}", c);
    match c {
        Ok(()) => println!("No Error"),
        Err(e) => println!("{}", e),
    }
    match erroch() {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    };
}

fn erroch() -> Result<String, Error> {
    let content = read_to_string("abc.txt").context("Failed to read the file")?;
    Ok(content)
}
