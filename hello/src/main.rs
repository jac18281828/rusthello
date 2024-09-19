use std::result::Result;

struct Errorf;

impl std::fmt::Display for Errorf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred!")
    }
}

const fn hello() -> Result<&'static str, Errorf> {
    let message = "Hello, world!";
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
