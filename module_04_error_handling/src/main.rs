mod exercises;

use thiserror::Error;

fn main() {
    result_basics();
    question_mark_operator();
    custom_errors();
    error_combinators();
}

// ---- Result basics ----

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn result_basics() {
    println!("--- Result Basics ---");

    // Explicit match
    match parse_number("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("abc") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // unwrap_or
    let n = parse_number("bad").unwrap_or(0);
    println!("unwrap_or: {}", n);

    // map
    let doubled = parse_number("21").map(|n| n * 2);
    println!("map doubled: {:?}", doubled);

    // is_ok / is_err
    println!("is_ok: {}", parse_number("5").is_ok());
    println!("is_err: {}", parse_number("x").is_err());
}

// ---- The ? operator ----

fn double_from_str(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = s.parse::<i32>()?; // returns early with Err if parse fails
    Ok(n * 2)
}

fn chain_operations(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let n = s.trim().parse::<i32>()?;
    let doubled = n.checked_mul(2).ok_or("overflow")?;
    Ok(format!("Result: {}", doubled))
}

fn question_mark_operator() {
    println!("\n--- ? Operator ---");

    println!("{:?}", double_from_str("21"));
    println!("{:?}", double_from_str("bad"));

    println!("{:?}", chain_operations("  100  "));
    println!("{:?}", chain_operations("bad"));
}

// ---- Custom error types ----

#[derive(Debug, Error)]
enum UserError {
    #[error("User not found with id: {id}")]
    NotFound { id: u32 },

    #[error("Invalid user id: '{raw}'")]
    InvalidId { raw: String },

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn get_user_score(user_id_str: &str) -> Result<u32, UserError> {
    let id: u32 = user_id_str
        .parse()
        .map_err(|_| UserError::InvalidId { raw: user_id_str.to_string() })?;

    if id == 0 {
        return Err(UserError::NotFound { id });
    }

    Ok(id * 10)
}

fn custom_errors() {
    println!("\n--- Custom Errors ---");

    println!("{:?}", get_user_score("5"));
    println!("{:?}", get_user_score("0"));
    println!("{:?}", get_user_score("abc"));

    // Display format (the #[error("...")] message)
    if let Err(e) = get_user_score("0") {
        println!("Display: {}", e);
    }
}

// ---- Combinators ----

fn error_combinators() {
    println!("\n--- Error Combinators ---");

    // and_then — chain Results
    let result = "5"
        .parse::<i32>()
        .and_then(|n| Ok(n * 2));
    println!("and_then: {:?}", result);

    // or_else — recover from an error
    let result = "bad"
        .parse::<i32>()
        .or_else(|_| Ok::<i32, std::num::ParseIntError>(0));
    println!("or_else: {:?}", result);

    // map_err — transform the error type
    let result: Result<i32, String> = "42"
        .parse::<i32>()
        .map_err(|e| format!("parse failed: {}", e));
    println!("map_err ok: {:?}", result);

    let result: Result<i32, String> = "bad"
        .parse::<i32>()
        .map_err(|e| format!("parse failed: {}", e));
    println!("map_err err: {:?}", result);

    // Collecting Results — all or nothing
    let strings = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("collect ok: {:?}", numbers);

    let mixed = vec!["1", "bad", "3"];
    let numbers: Result<Vec<i32>, _> = mixed.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("collect err: {:?}", numbers);
}
