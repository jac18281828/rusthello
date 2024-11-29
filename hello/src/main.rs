use std::result::Result;

use thiserror::Error;

const HELLO: &str = "Hello, world!";

#[allow(dead_code)]
#[derive(Debug, Error)]
enum Errorf {
    #[error("Unknown result")]
    UnknownResultError(String),
}

const fn hello() -> Result<&'static str, Errorf> {
    let message = HELLO;
    Ok(message)
}

fn main() {
    match hello() {
        Ok(msg) => println!("{}", msg),
        Err(error) => println!("Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let result = hello();
        match result {
            Ok(message) => assert_eq!(message, "Hello, world!"),
            Err(_) => panic!("Test failed!"),
        }
    }
}
