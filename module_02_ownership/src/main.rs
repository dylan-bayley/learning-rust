mod exercises;

fn main() {
    move_semantics();
    copy_types();
    references_and_borrowing();
    mutable_references();
    string_vs_str();
    slices();
}

fn move_semantics() {
    println!("--- Move Semantics ---");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2
    // println!("{}", s1);  // would be a compile error: value moved
    println!("s2 = {}", s2);

    // To keep s1 valid, clone it
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // both valid

    // Moving into a function
    let s = String::from("world");
    takes_ownership(s);
    // println!("{}", s);  // compile error: s was moved

    // Getting ownership back
    let s5 = String::from("returned");
    let s6 = gives_back(s5);
    println!("Got back: {}", s6);
}

fn takes_ownership(s: String) {
    println!("Got: {}", s);
} // s is dropped here

fn gives_back(s: String) -> String {
    s // move s out to the caller
}

fn copy_types() {
    println!("\n--- Copy Types ---");

    // Integers, floats, bools, chars are Copy — assignment copies, not moves
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // both valid — x was copied

    let a = 3.14;
    let b = a;
    println!("a = {}, b = {}", a, b);
}

fn print_length(s: &String) { // borrows s, does not take ownership
    println!("Length of '{}' is {}", s, s.len());
}

fn references_and_borrowing() {
    println!("\n--- References and Borrowing ---");

    let name = String::from("Dylan");
    print_length(&name); // pass a reference — name is still valid
    println!("name is still: {}", name);

    // Multiple immutable references are allowed
    let r1 = &name;
    let r2 = &name;
    println!("r1 = {}, r2 = {}", r1, r2);
}

fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

fn mutable_references() {
    println!("\n--- Mutable References ---");

    let mut greeting = String::from("Hello");
    append_exclamation(&mut greeting);
    println!("{}", greeting); // "Hello!"

    // Only one mutable reference at a time
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1 goes out of scope here

    let r2 = &mut s; // now fine — r1 no longer exists
    println!("{}", r2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

fn string_vs_str() {
    println!("\n--- String vs &str ---");

    let owned: String = String::from("owned string");
    let literal: &str = "string literal";

    // &String coerces to &str automatically
    println!("first word of owned: '{}'", first_word(&owned));
    println!("first word of literal: '{}'", first_word(literal));
}

fn slices() {
    println!("\n--- Slices ---");

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // Range shorthand
    let first_three = &s[..3]; // same as &s[0..3]
    let last_five = &s[6..];   // to end
    println!("{} {}", first_three, last_five);

    // Array/Vec slices
    let numbers = vec![1, 2, 3, 4, 5];
    let middle = &numbers[1..4];
    println!("middle: {:?}", middle);

    // Slices work with any contiguous sequence
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..3];
    println!("array slice: {:?}", slice);
}
