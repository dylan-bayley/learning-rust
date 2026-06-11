mod exercises;

fn main() {
    variables_and_mutability();
    basic_types();
    functions_demo();
    control_flow_demo();
}

fn variables_and_mutability() {
    println!("--- Variables and Mutability ---");

    let x = 5;
    println!("x = {}", x);

    // x = 6;  // would be a compile error

    let mut y = 5;
    y = 10;
    println!("y = {}", y);

    // Shadowing — rebind with let, can change type
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z (shadowed) = {}", z);

    let spaces = "   ";
    let spaces = spaces.len(); // shadows with a different type — fine with shadowing
    println!("spaces length = {}", spaces);
}

fn basic_types() {
    println!("\n--- Basic Types ---");

    let age: i32 = 30;
    let temperature: f64 = 36.6;
    let is_active: bool = true;
    let initial: char = 'D';
    let name: String = String::from("Dylan");
    let greeting: &str = "hello";

    println!("age: {}", age);
    println!("temperature: {}", temperature);
    println!("is_active: {}", is_active);
    println!("initial: {}", initial);
    println!("name: {}", name);
    println!("greeting: {}", greeting);

    // Arithmetic
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("\nArithmetic: {}, {:.2}, {}, {:.2}, {}", sum, difference, product, quotient, remainder);

    // Tuples
    let tup: (i32, f64, bool) = (42, 3.14, true);
    let (a, b, c) = tup; // destructuring
    println!("\nTuple destructured: {}, {}, {}", a, b, c);
    println!("Tuple index access: {}", tup.0);

    // Arrays (fixed size, same type — different from Vec)
    let arr = [1, 2, 3, 4, 5];
    println!("\nArray: {:?}", arr);
    println!("First element: {}", arr[0]);
    println!("Length: {}", arr.len());
}

fn add(x: i32, y: i32) -> i32 {
    x + y // no semicolon = implicit return
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name) // format! is like f-string
}

fn absolute_value(n: i32) -> i32 {
    if n < 0 { -n } else { n } // if as an expression
}

fn functions_demo() {
    println!("\n--- Functions ---");
    println!("add(3, 4) = {}", add(3, 4));
    println!("greet = {}", greet("Dylan"));
    println!("abs(-7) = {}", absolute_value(-7));
    println!("abs(5) = {}", absolute_value(5));
}

fn control_flow_demo() {
    println!("\n--- Control Flow ---");

    // if / else
    let number = 7;
    if number < 0 {
        println!("{} is negative", number);
    } else if number == 0 {
        println!("zero");
    } else {
        println!("{} is positive", number);
    }

    // for loop with range
    print!("Range 0..5: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // for over a collection
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("  fruit: {}", fruit);
    }

    // while
    let mut counter = 0;
    while counter < 3 {
        print!("{} ", counter);
        counter += 1;
    }
    println!();

    // loop with break value
    let mut x = 0;
    let result = loop {
        x += 1;
        if x == 5 {
            break x * 10;
        }
    };
    println!("loop result: {}", result);

    // Nested loop with labels
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                break 'outer; // break the outer loop
            }
            print!("({},{}) ", i, j);
        }
    }
    println!();
}
