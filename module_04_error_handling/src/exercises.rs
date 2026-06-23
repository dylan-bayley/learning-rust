// Module 04 Exercises — Error Handling
//
// Implement each function to make the tests pass.
// Run:  cargo test -p module_04_error_handling

use thiserror::Error;

// ---- Exercise 1: Working with Result ----------------------------------------
//
// Parse a string as an integer and double it.
// Return an Err if parsing fails.
pub fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?; // the ? replaces the requirement for a match statement, which means if its okay, itll pass back the okay value, if not, pass back the error. 
    Ok(n*2)
}

// ---- Exercise 2: Chaining with ? --------------------------------------------
//
// Parse two strings as integers and return their sum.
// Return an Err if either fails to parse.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let a_i: i32 = a.parse()?;
    let b_i: i32 = b.parse()?;
    Ok(a_i + b_i)
}

// ---- Exercise 3: Custom error type ------------------------------------------
//
// Define a MathError enum with two variants:
//   - DivisionByZero
//   - NegativeSquareRoot(f64)  — the value that was negative
//
// Both should have human-readable messages via thiserror.

#[derive(Debug, Error, PartialEq)]
pub enum MathError {
    #[error("Division by zero")]  // this derive passes back the Error message
    DivisionByZero,
    #[error("Negative Square Root: '{0}'")] // note that this message expects a value to be returned with the error
    NegativeSquareRoot(f64),
}

// Divide a by b. Return MathError::DivisionByZero if b == 0.0
pub fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

// Return the square root of x. Return MathError::NegativeSquareRoot(x) if x < 0.
pub fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        return Err(MathError::NegativeSquareRoot(x));
    }
    Ok(x.sqrt())
}

// ---- Exercise 4: Error propagation ------------------------------------------
//
// Parse a string as f64, then take its square root.
// Use ? to propagate errors. You'll need to handle two different error types.
// Hint: use map_err to convert the parse error into a String, then Box or a shared type.
//
// Return type uses Box<dyn std::error::Error> to accept multiple error types.
pub fn parse_and_sqrt(s: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let num: f64 = s.parse().map_err(|e: std::num::ParseFloatError|e.to_string())?;
    if num < 0.0 {
        return Err(MathError::NegativeSquareRoot(num).into());
    }
    Ok(num.sqrt())
}

// ---- Exercise 5: Result combinators -----------------------------------------
//
// Given a Vec of strings, parse each as i32. Return a Vec of only the
// successfully parsed values (skip the failures — don't return an error).
pub fn parse_all_ok(strings: &[&str]) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for s in strings {
        if let Ok(num) = s.parse::<i32>() {
            v.push(num);
        }
    }
    v
}

// ---- Tests ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("21"), Ok(42));
        assert_eq!(parse_and_double("0"), Ok(0));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_parse_and_add() {
        assert_eq!(parse_and_add("3", "4"), Ok(7));
        assert_eq!(parse_and_add("0", "0"), Ok(0));
        assert!(parse_and_add("a", "4").is_err());
        assert!(parse_and_add("3", "b").is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(7.0, 0.0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_safe_sqrt() {
        let result = safe_sqrt(4.0).unwrap();
        assert!((result - 2.0).abs() < 1e-9);

        assert_eq!(safe_sqrt(-1.0), Err(MathError::NegativeSquareRoot(-1.0)));
    }

    #[test]
    fn test_math_error_display() {
        // Check that error messages are human-readable
        let e = MathError::DivisionByZero;
        assert!(!e.to_string().is_empty());

        let e2 = MathError::NegativeSquareRoot(-4.0);
        assert!(e2.to_string().contains("-4"));
    }

    #[test]
    fn test_parse_and_sqrt() {
        let result = parse_and_sqrt("4").unwrap();
        assert!((result - 2.0).abs() < 1e-9);

        let result = parse_and_sqrt("9").unwrap();
        assert!((result - 3.0).abs() < 1e-9);

        assert!(parse_and_sqrt("abc").is_err());
    }

    #[test]
    fn test_parse_all_ok() {
        assert_eq!(parse_all_ok(&["1", "2", "3"]), vec![1, 2, 3]);
        assert_eq!(parse_all_ok(&["1", "bad", "3"]), vec![1, 3]);
        assert_eq!(parse_all_ok(&["bad", "also bad"]), Vec::<i32>::new());
        assert_eq!(parse_all_ok(&[]), Vec::<i32>::new());
    }
}
