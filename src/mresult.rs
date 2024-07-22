#![allow(unused)]

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        matches!(self, MResult::Ok(_))
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        matches!(self, MResult::Err(_))
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value, 
            MResult::Err(_) => panic!("called unwrap on an Err value"), 
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Err(error) => error, 
            MResult::Ok(_) => panic!("called unwrap_err on an Ok value"), 
        }
    }
}

// Add unit tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "error");
    }
}

