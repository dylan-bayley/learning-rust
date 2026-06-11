mod exercises;

fn main() {
    structs_demo();
    enums_demo();
    pattern_matching_demo();
    option_demo();
}

// ---- Structs ----

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // Associated function (no self) — called as Rectangle::unit()
    fn unit() -> Rectangle {
        Rectangle { width: 1.0, height: 1.0 }
    }
}

fn structs_demo() {
    println!("--- Structs ---");

    let rect = Rectangle { width: 10.0, height: 5.0 };
    println!("{:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());

    let mut r2 = Rectangle { width: 3.0, height: 3.0 };
    println!("r2 is square: {}", r2.is_square());
    r2.scale(2.0);
    println!("After scale(2): {:?}", r2);

    let unit = Rectangle::unit();
    println!("Unit rectangle: {:?}", unit);
}

// ---- Enums ----

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle { .. } => "triangle",
        }
    }
}

fn enums_demo() {
    println!("\n--- Enums ---");

    let coin = Coin::Quarter;
    println!("{:?} = {} cents", coin, coin.value_in_cents());

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 8.0 },
    ];

    for shape in &shapes {
        println!("{} area: {:.2}", shape.name(), shape.area());
    }
}

// ---- Pattern Matching ----

fn describe_number(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=9 => "small positive",
        10..=99 => "medium positive",
        _ => "large positive",
    }
}

fn pattern_matching_demo() {
    println!("\n--- Pattern Matching ---");

    for n in [-5, 0, 3, 42, 150] {
        println!("{}: {}", n, describe_number(n));
    }

    // Destructuring in match
    let point = (3, -2);
    let quadrant = match point {
        (x, y) if x > 0 && y > 0 => "Q1",
        (x, y) if x < 0 && y > 0 => "Q2",
        (x, y) if x < 0 && y < 0 => "Q3",
        (x, y) if x > 0 && y < 0 => "Q4",
        _ => "on an axis",
    };
    println!("Point {:?} is in {}", point, quadrant);
}

// ---- Option ----

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn option_demo() {
    println!("\n--- Option ---");

    // match
    match find_first_even(&[1, 3, 4, 6]) {
        Some(n) => println!("Found even: {}", n),
        None => println!("No even numbers"),
    }

    match find_first_even(&[1, 3, 5]) {
        Some(n) => println!("Found even: {}", n),
        None => println!("No even numbers"),
    }

    // if let
    if let Some(n) = find_first_even(&[7, 8, 9]) {
        println!("if let found: {}", n);
    }

    // unwrap_or
    let result = find_first_even(&[1, 3, 5]).unwrap_or(-1);
    println!("unwrap_or: {}", result);

    // map
    let doubled = find_first_even(&[1, 2, 3]).map(|n| n * 2);
    println!("map doubled: {:?}", doubled);

    // chaining with and_then
    let parsed: Option<i32> = Some("42").and_then(|s| s.parse().ok());
    println!("parsed: {:?}", parsed);
}
