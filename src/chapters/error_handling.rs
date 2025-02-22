use core::fmt;
use std::error::Error;
#[derive(Debug)]
enum OperationError {
    DivideByZeroError,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OperationError::DivideByZeroError => f.write_str("failed to DivideByZeroErrors")
        }    
    }
}

impl Error for OperationError {
    fn description(&self) -> &str {
        match *self {
            OperationError::DivideByZeroError => "Cannot Divide By Zero"
        }
    }
}


pub fn error_handling() {
    let a = 100;
    let b = 0;
    let c = divide(a, b);
    match c {
        Ok(result) => println!("Result is {}",result);
        Err(e) => println!("Error is {}",e)
    }
    let d = OperationError::DivideByZeroError;
    println!("{:?}", d);
    println!("{:?}", c);
}

fn divide(x: u32, y: u32) -> Result<u32, OperationError> {
    if y == 0 {
        Err(OperationError::DivideByZeroError)
    } else {
        Ok(x / y)
    }
}
