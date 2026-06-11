# Module 03: Structs and Enums

Rust doesn't have classes. Instead it has structs (data) and `impl` blocks (methods), plus enums that can carry data. Together these are more expressive than Python classes in many ways.

---

## 1. Structs

```python
# Python
class Rectangle:
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height
```

```rust
// Rust
struct Rectangle {
    width: f64,
    height: f64,
}
```

Creating an instance:

```rust
let rect = Rectangle { width: 10.0, height: 5.0 };
println!("{}", rect.width);  // field access, same as Python
```

### Methods with `impl`

```python
# Python
class Rectangle:
    def area(self) -> float:
        return self.width * self.height
    
    @staticmethod
    def unit_square() -> "Rectangle":
        return Rectangle(1.0, 1.0)
```

```rust
// Rust
impl Rectangle {
    // Method — takes &self (like self in Python)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn scale(&mut self, factor: f64) {  // needs &mut self to modify
        self.width *= factor;
        self.height *= factor;
    }

    // Associated function (like @staticmethod) — no self
    fn unit_square() -> Rectangle {
        Rectangle { width: 1.0, height: 1.0 }
    }
}

let rect = Rectangle { width: 10.0, height: 5.0 };
println!("Area: {}", rect.area());
let unit = Rectangle::unit_square();  // called on the type, not an instance
```

### Debug printing

To print a struct with `{:?}`, derive the `Debug` trait:

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

println!("{:?}", rect);   // Rectangle { width: 10.0, height: 5.0 }
println!("{:#?}", rect);  // pretty-printed
```

---

## 2. Enums

Python enums are mostly just named constants. Rust enums can carry data — they're more like algebraic types.

```rust
// Simple enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with data — each variant can have different data
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}
```

---

## 3. Pattern Matching

`match` is like Python's `match`/`case` (Python 3.10+) but far more powerful.

```rust
let dir = Direction::North;

match dir {
    Direction::North => println!("Going north"),
    Direction::South => println!("Going south"),
    Direction::East | Direction::West => println!("Going sideways"),
}
```

Matching enums with data:

```rust
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}
```

`match` must be **exhaustive** — you must cover every variant (or use `_` as a catch-all).

---

## 4. Option<T>

Rust has no `None` floating around like Python. Instead, the *possibility of absence* is encoded in the type: `Option<T>`.

```python
# Python — None can sneak anywhere
def find_user(id: int) -> User | None:
    ...
```

```rust
// Rust — the type forces you to handle the None case
fn find_user(id: u32) -> Option<User> {
    ...
}
```

`Option<T>` is just an enum:
```rust
enum Option<T> {
    Some(T),  // there is a value
    None,     // there is no value
}
```

Working with `Option`:

```rust
let maybe_name: Option<String> = Some(String::from("Dylan"));
let nothing: Option<i32> = None;

// Pattern match (most explicit)
match maybe_name {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}

// if let (cleaner when you only care about Some)
if let Some(name) = &maybe_name {
    println!("Found: {}", name);
}

// unwrap_or — provide a default
let name = maybe_name.unwrap_or(String::from("Unknown"));

// map — transform the inner value if it exists
let length = maybe_name.map(|s| s.len());  // Option<usize>
```

---

## 5. if let

A shorthand for matching a single pattern:

```rust
// Instead of this:
match some_option {
    Some(value) => do_something(value),
    None => {}  // awkward empty arm
}

// Write this:
if let Some(value) = some_option {
    do_something(value);
}
```

---

## Exercises

Open `src/exercises.rs` and implement each function.

```bash
cargo test -p module_03_structs_enums
```
