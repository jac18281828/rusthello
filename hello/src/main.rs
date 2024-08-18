use anyhow::Result;

const fn hello() -> Result<&'static str> {
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
