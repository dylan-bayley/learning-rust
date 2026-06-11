# Module 02: Ownership

This is the most important module. Ownership is what makes Rust unique — it's the system that gives you memory safety without a garbage collector. Most Rust frustration for beginners comes from fighting the borrow checker. After this module, you'll work *with* it.

---

## 1. Why Ownership Exists

In Python, a garbage collector tracks which objects are still referenced and frees memory automatically. This is safe but slow. In C/C++, you manage memory manually — fast but dangerous (use-after-free bugs, memory leaks).

Rust's answer: a set of compile-time rules called **ownership**. The compiler proves your code is memory-safe before it ever runs.

---

## 2. The Three Rules

1. Each value has exactly **one owner**
2. There can only be **one owner at a time**
3. When the owner goes out of scope, the value is **dropped** (memory freed)

---

## 3. Move Semantics

```python
# Python — both a and b refer to the same list
a = [1, 2, 3]
b = a
print(a)  # [1, 2, 3] — still works
```

```rust
// Rust — ownership of the Vec moves to b
let a = vec![1, 2, 3];
let b = a;
// println!("{:?}", a);  // ERROR: value moved, a is no longer valid
println!("{:?}", b);  // fine
```

When you assign a heap-allocated value to another variable, ownership **moves**. The original variable is invalidated. This prevents double-free bugs.

### Copy types

Small, stack-allocated types (integers, floats, bools, chars) are **copied** instead of moved — they're cheap to duplicate:

```rust
let x = 5;
let y = x;
println!("{}", x);  // fine — integers are Copy
println!("{}", y);  // fine
```

---

## 4. References and Borrowing

What if you want to use a value without taking ownership? You **borrow** it with a reference.

```rust
fn print_length(s: &String) {  // & means "reference to"
    println!("Length: {}", s.len());
}  // s goes out of scope but doesn't drop the String — it's just a borrow

let name = String::from("Dylan");
print_length(&name);  // pass a reference
println!("{}", name);  // name is still valid — we only lent it
```

Think of it like lending a book: the library still owns it, you just have a reference to read it.

### Rules of References

1. You can have **any number of immutable references** (`&T`) at the same time
2. OR you can have **exactly one mutable reference** (`&mut T`)
3. But **not both at the same time**

This prevents data races at compile time.

```rust
let mut s = String::from("hello");

let r1 = &s;      // immutable borrow
let r2 = &s;      // another immutable borrow — fine
// let r3 = &mut s;  // ERROR: can't borrow mutably while immutable borrows exist

println!("{} {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s;  // now fine — r1 and r2 are no longer active
r3.push_str(", world");
```

### Mutable references

```rust
fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

let mut greeting = String::from("Hello");
append_exclamation(&mut greeting);
println!("{}", greeting);  // "Hello!"
```

---

## 5. String vs &str

This is where the theory becomes practical:

- `String` — **owned**, heap-allocated, can grow/shrink. Like Python's `str` in memory terms.
- `&str` — **borrowed slice** of string data. Points into existing string data. Can't be modified.

```rust
let owned: String = String::from("hello");   // heap allocation
let literal: &str = "world";                 // baked into the binary

// &String can be coerced to &str automatically
fn greet(name: &str) {  // prefer &str in function params — works with both
    println!("Hello, {}", name);
}

greet("literal");              // &str — works
greet(&owned);                 // &String coerces to &str — also works
```

**Rule of thumb for function parameters:** accept `&str` instead of `&String` — it's more flexible.

---

## 6. Slices

A slice is a reference to a contiguous sequence of elements in a collection.

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // &str slice: "hello"
let world = &s[6..11];  // &str slice: "world"

let numbers = vec![1, 2, 3, 4, 5];
let middle = &numbers[1..4];  // [2, 3, 4]
```

---

## 7. Lifetimes (Just Enough)

Lifetimes are how Rust tracks how long references are valid. The compiler usually figures them out automatically. You only need to annotate them when the compiler can't infer them — most commonly when a function returns a reference.

```rust
// The compiler needs to know: does the return value come from x or y?
// 'a means "the return lives at least as long as both inputs"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Don't worry about mastering lifetimes yet — focus on ownership and borrowing first.

---

## Exercises

Open `src/exercises.rs`. Implement each function to make the tests pass.

```bash
cargo test -p module_02_ownership
```
