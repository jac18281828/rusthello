use std::io;

fn hello() -> Result<String, io::Error> {
    let message = "Hello, world!";
    println!("{}", message);
    Ok(message.to_string())
}

fn main() {
    let result = hello();
    match result {
        Ok(_) => {}
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

    #[test]
    fn panichello() {
        panic!("Make this test fail");
    }
}
