# Module 04: Error Handling

Python uses exceptions. Rust uses return values. This sounds limiting but it's actually more robust — errors become part of the function's type signature, so the compiler forces you to handle them.

---

## 1. Result<T, E>

`Result` is an enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),   // success, contains a value of type T
    Err(E),  // failure, contains an error of type E
}
```

```python
# Python — no indication in the signature that this can fail
def parse_age(s: str) -> int:
    return int(s)  # raises ValueError silently
```

```rust
// Rust — the return type advertises the failure case
fn parse_age(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}
```

Using it:

```rust
match parse_age("42") {
    Ok(age) => println!("Age: {}", age),
    Err(e) => println!("Invalid: {}", e),
}
```

---

## 2. The ? Operator

Writing `match` everywhere gets verbose. The `?` operator is shorthand for "return the error if this failed, otherwise unwrap the Ok value":

```python
# Python
def read_and_parse(filename: str) -> int:
    with open(filename) as f:      # can raise FileNotFoundError
        content = f.read().strip() # 
    return int(content)            # can raise ValueError
```

```rust
// Rust — ? propagates errors up to the caller
use std::fs;
use std::num::ParseIntError;

fn read_and_parse(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;  // ? on io::Error
    let number = content.trim().parse::<i32>()?;  // ? on ParseIntError
    Ok(number)
}
```

`?` can only be used in functions that return `Result` (or `Option`).

---

## 3. Custom Error Types with thiserror

For real applications you want a single custom error type rather than `Box<dyn Error>`. The `thiserror` crate makes this easy.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("User not found: {id}")]
    UserNotFound { id: u32 },

    #[error("Database error: {0}")]
    Database(String),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),  // #[from] auto-converts ParseIntError
}
```

The `#[error("...")]` attribute defines the Display message. `#[from]` automatically implements `From<ParseIntError>` so `?` converts it.

```rust
fn get_user_score(user_id: &str) -> Result<u32, AppError> {
    let id: u32 = user_id.parse()?;  // ParseIntError auto-converts to AppError::Parse
    if id == 0 {
        return Err(AppError::UserNotFound { id });
    }
    Ok(id * 10)
}
```

---

## 4. Handling Errors at the Call Site

```rust
// unwrap() — panics if Err. Use only in tests or prototyping.
let val = some_result.unwrap();

// expect() — panics with a message. Better than unwrap for debugging.
let val = some_result.expect("should have a valid value here");

// unwrap_or — provide a default
let val = some_result.unwrap_or(0);

// unwrap_or_else — compute a default lazily
let val = some_result.unwrap_or_else(|e| {
    eprintln!("Error: {}", e);
    0
});

// map — transform Ok value, leave Err unchanged
let doubled = some_result.map(|n| n * 2);

// and_then — chain fallible operations
let result = parse_age("42")
    .and_then(|age| if age < 150 { Ok(age) } else { Err(...) });
```

---

## 5. Error Handling Strategy

| Situation | What to use |
|-----------|-------------|
| Library code | Custom error type with `thiserror` |
| Application entry point | `Box<dyn Error>` or `anyhow` |
| Definitely won't fail | `.expect("reason why")` |
| Tests | `.unwrap()` is fine |
| Propagating up | `?` |

---

## Exercises

Open `src/exercises.rs` and implement each function.

```bash
cargo test -p module_04_error_handling
```
