# Module 05: Traits and Iterators

Traits are Rust's answer to interfaces, abstract base classes, and duck typing. Iterators are how Rust does functional-style data transformation — think Python's `map`, `filter`, `sum` but composable and zero-cost.

---

## 1. Traits

```python
# Python — duck typing or ABC
from abc import ABC, abstractmethod

class Describable(ABC):
    @abstractmethod
    def describe(self) -> str: ...
```

```rust
// Rust — explicit trait definition
trait Describable {
    fn describe(&self) -> String;
}
```

Implementing a trait:

```rust
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Describable for Dog {
    fn describe(&self) -> String {
        format!("{} is a dog", self.name)
    }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        format!("{} is a cat", self.name)
    }
}
```

### Default implementations

```rust
trait Greet {
    fn name(&self) -> &str;

    // Default implementation — can be overridden
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name())
    }
}
```

---

## 2. Standard Traits You'll Use Constantly

### Debug and Display

```rust
use std::fmt;

#[derive(Debug)]  // derive gives you {:?} printing for free
struct Point {
    x: f64,
    y: f64,
}

// Display is for human-readable output (like Python's __str__)
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1.0, y: 2.0 };
println!("{:?}", p);  // Debug: Point { x: 1.0, y: 2.0 }
println!("{}", p);    // Display: (1.0, 2.0)
```

### Clone and Copy

```rust
#[derive(Debug, Clone)]  // Clone enables .clone()
struct Config {
    max_connections: u32,
}

let c1 = Config { max_connections: 10 };
let c2 = c1.clone();  // explicit deep copy
```

---

## 3. Trait Bounds (Generics)

```python
# Python — type hints are checked at runtime or by mypy
def largest(items: list[int | float]) -> int | float:
    ...
```

```rust
// Rust — generic function, T must implement PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

// Multiple bounds with +
fn print_largest<T: PartialOrd + std::fmt::Display>(list: &[T]) {
    println!("Largest: {}", largest(list));
}
```

### `impl Trait` syntax (cleaner for simple cases)

```rust
fn greet_all(items: &[impl Describable]) {
    for item in items {
        println!("{}", item.describe());
    }
}
```

---

## 4. Iterators

Python's iteration model and Rust's are conceptually similar, but Rust's is zero-cost and lazy by default.

```python
# Python
numbers = [1, 2, 3, 4, 5, 6]
result = sum(x ** 2 for x in numbers if x % 2 == 0)
```

```rust
// Rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let result: i32 = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)   // keep even numbers
    .map(|&x| x * x)             // square them
    .sum();                       // sum them up
```

### Iterator methods

| Rust | Python equivalent | Notes |
|------|------------------|-------|
| `.map(f)` | `map(f, items)` | Transform each element |
| `.filter(pred)` | `filter(pred, items)` | Keep matching elements |
| `.sum()` | `sum(items)` | Sum all elements |
| `.count()` | `len(list(items))` | Count elements |
| `.collect::<Vec<_>>()` | `list(...)` | Gather into a Vec |
| `.enumerate()` | `enumerate(items)` | Pairs of (index, value) |
| `.zip(other)` | `zip(a, b)` | Pair two iterators |
| `.fold(init, f)` | `functools.reduce` | Reduce with accumulator |
| `.any(pred)` | `any(pred(x) for x in items)` | True if any match |
| `.all(pred)` | `all(pred(x) for x in items)` | True if all match |
| `.find(pred)` | `next(x for x in items if pred(x), None)` | First match as Option |
| `.flat_map(f)` | `itertools.chain.from_iterable(map(f, items))` | Map then flatten |

### .iter() vs .into_iter() vs .iter_mut()

```rust
let v = vec![1, 2, 3];

v.iter()         // yields &i32 — borrows elements, v still valid after
v.into_iter()    // yields i32  — consumes v (moves ownership)
v.iter_mut()     // yields &mut i32 — borrows mutably, lets you modify in place
```

For most read-only operations, use `.iter()`.

### collect()

```rust
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|&x| x * 2)
    .collect();  // like list() in Python — you must annotate the target type

// Collect into a HashMap
use std::collections::HashMap;
let map: HashMap<&str, i32> = vec![("a", 1), ("b", 2)]
    .into_iter()
    .collect();
```

---

## 5. Closures

Closures are anonymous functions that capture their environment — like Python's lambdas but can span multiple lines.

```python
# Python
double = lambda x: x * 2
add = lambda x, y: x + y
```

```rust
// Rust
let double = |x| x * 2;
let add = |x, y| x + y;

// Can capture from surrounding scope
let factor = 3;
let multiply = |x| x * factor;  // captures `factor`
```

---

## Exercises

Open `src/exercises.rs` and implement each function.

```bash
cargo test -p module_05_traits_iterators
```
