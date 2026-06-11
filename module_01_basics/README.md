# Module 01: Basics

## What You'll Learn
- The Cargo build system
- Variables, mutability, and basic types
- Functions
- Control flow: if/else, loop, while, for

---

## 1. Cargo

Cargo is Rust's package manager and build tool — think `pip` + `poetry` + the Python interpreter bundled together.

```bash
cargo new my_project   # like `mkdir my_project && touch main.py`
cargo run              # compile + run
cargo build            # just compile
cargo check            # check for errors without building (fast)
cargo test             # run tests
```

---

## 2. Variables and Mutability

In Python everything is mutable by default. In Rust, **variables are immutable by default**. You opt into mutability with `mut`.

```python
# Python
x = 5
x = 10  # fine
```

```rust
// Rust
let x = 5;
// x = 10;  // ERROR: cannot assign twice to immutable variable

let mut y = 5;
y = 10;  // fine
```

Why? Immutability by default catches a whole class of bugs at compile time.

### Shadowing

You can re-declare a variable with `let` — this is called *shadowing*:

```rust
let x = 5;
let x = x + 1;  // x is now 6, this is a new binding
let x = x * 2;  // x is now 12
```

This is different from `mut` — you can even change the type when shadowing.

---

## 3. Basic Types

| Rust | Python equivalent | Notes |
|------|------------------|-------|
| `i32` | `int` | 32-bit signed integer (default int) |
| `i64` | `int` | 64-bit signed integer |
| `u32` | `int` (non-negative) | 32-bit unsigned integer |
| `f64` | `float` | 64-bit float (default float) |
| `bool` | `bool` | `true` / `false` (lowercase) |
| `char` | `str` of length 1 | Unicode scalar, single quotes: `'a'` |
| `String` | `str` | Owned, heap-allocated, mutable string |
| `&str` | `str` | Borrowed string slice (more on this in module 02) |

```rust
let age: i32 = 30;
let price: f64 = 9.99;
let is_active: bool = true;
let initial: char = 'D';
let name: String = String::from("Dylan");
let greeting: &str = "hello";
```

Rust can usually infer types, so you often don't need the annotation:

```rust
let age = 30;       // Rust infers i32
let price = 9.99;   // Rust infers f64
```

### Strings: `String` vs `&str`

This trips up Python developers. For now, the rule of thumb:
- Use `&str` for string literals and function parameters you only need to read
- Use `String` when you need to own or modify a string

We'll go deep on this in module 02.

---

## 4. Functions

```python
# Python
def add(x: int, y: int) -> int:
    return x + y
```

```rust
// Rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // no semicolon = this is the return value
}
```

Key differences:
- Parameter types are **required** (no dynamic typing)
- The last expression in a block (without `;`) is automatically returned
- You can also use `return x + y;` explicitly

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)  // like f"Hello, {name}!"
}
```

---

## 5. Control Flow

### if / else

```python
# Python
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")
```

```rust
// Rust
if x > 0 {
    println!("positive");
} else if x < 0 {
    println!("negative");
} else {
    println!("zero");
}
```

In Rust, `if` is an **expression** — it returns a value:

```rust
let description = if x > 0 { "positive" } else { "non-positive" };
// like Python's: description = "positive" if x > 0 else "non-positive"
```

### Loops

```rust
// Counting loop (like Python's range)
for i in 0..5 {        // 0, 1, 2, 3, 4
    println!("{}", i);
}

for i in 0..=5 {       // 0, 1, 2, 3, 4, 5 (inclusive)
    println!("{}", i);
}

// Iterating a collection
let numbers = vec![1, 2, 3, 4, 5];  // vec! is like Python's list
for n in &numbers {
    println!("{}", n);
}

// while
let mut count = 0;
while count < 5 {
    count += 1;
}

// loop (infinite, exit with break)
let mut x = 0;
let result = loop {
    x += 1;
    if x == 10 {
        break x * 2;  // break can return a value
    }
};
```

---

## 6. Printing

```rust
println!("Hello, world!");          // like print()
println!("Value: {}", x);           // like print(f"Value: {x}")
println!("x={}, y={}", x, y);       // multiple values
println!("Debug: {:?}", some_vec);  // debug format (prints internals)
```

---

## Exercises

Open `src/exercises.rs`. Implement each function to make the tests pass.

```bash
cargo test -p module_01_basics
```

Work through them in order — each one builds on the concepts above.
