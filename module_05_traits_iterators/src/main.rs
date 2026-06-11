mod exercises;

use std::fmt;

fn main() {
    traits_demo();
    generics_demo();
    iterators_demo();
    closures_demo();
}

// ---- Traits ----

trait Area {
    fn area(&self) -> f64;
}

trait Perimeter {
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Perimeter for Square {
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

// Trait with a default method
trait Describe: Area {
    fn name(&self) -> &str;

    fn describe(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

impl Describe for Circle {
    fn name(&self) -> &str { "circle" }
}

impl Describe for Square {
    fn name(&self) -> &str { "square" }
}

// Custom Display implementation
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn traits_demo() {
    println!("--- Traits ---");

    let c = Circle { radius: 3.0 };
    let s = Square { side: 4.0 };

    println!("Circle: {}", c.describe());
    println!("Square: {}", s.describe());

    // Dynamic dispatch — store different types behind a trait object
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
        Box::new(Circle { radius: 5.0 }),
    ];

    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("Total area: {:.2}", total_area);

    let p = Point { x: 3.0, y: 4.0 };
    println!("Point Display: {}", p);
    println!("Point Debug: {:?}", p);
}

// ---- Generics ----

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

fn print_all<T: fmt::Display>(items: &[T]) {
    for item in items {
        print!("{} ", item);
    }
    println!();
}

fn generics_demo() {
    println!("\n--- Generics ---");

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    print!("Numbers: ");
    print_all(&numbers);
    print!("Chars: ");
    print_all(&chars);
}

// ---- Iterators ----

fn iterators_demo() {
    println!("\n--- Iterators ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter + map + sum
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("Sum of squares of even numbers: {}", sum_of_even_squares);

    // collect into a new Vec
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // enumerate
    let fruits = vec!["apple", "banana", "cherry"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", i, fruit);
    }

    // zip
    let a = vec![1, 2, 3];
    let b = vec!["one", "two", "three"];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("Zipped: {:?}", zipped);

    // fold (like Python's functools.reduce)
    let product = vec![1, 2, 3, 4, 5]
        .iter()
        .fold(1, |acc, &x| acc * x);
    println!("Product via fold: {}", product);

    // any / all
    println!("any > 5: {}", numbers.iter().any(|&x| x > 5));
    println!("all > 0: {}", numbers.iter().all(|&x| x > 0));

    // find
    println!("first > 5: {:?}", numbers.iter().find(|&&x| x > 5));

    // flat_map
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("flat_map words: {:?}", chars);

    // chain
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let combined: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    println!("chained: {:?}", combined);
}

// ---- Closures ----

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // `move` captures n by value
}

fn closures_demo() {
    println!("\n--- Closures ---");

    let double = |x| x * 2;
    let add_three = |x| x + 3;

    println!("double 5: {}", double(5));
    println!("apply_twice double 3: {}", apply_twice(double, 3));
    println!("apply_twice add_three 0: {}", apply_twice(add_three, 0));

    // Closure capturing environment
    let threshold = 5;
    let is_big = |x: &i32| *x > threshold;
    let big_numbers: Vec<&i32> = vec![1, 10, 3, 8, 2].iter().filter(|x| is_big(x)).collect();
    println!("big numbers: {:?}", big_numbers);

    // Returning a closure
    let add_ten = make_adder(10);
    println!("add_ten(5): {}", add_ten(5));
    println!("add_ten(20): {}", add_ten(20));
}
