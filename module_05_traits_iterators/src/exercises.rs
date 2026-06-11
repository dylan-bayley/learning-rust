// Module 05 Exercises — Traits and Iterators
//
// Implement each function/trait to make the tests pass.
// Run:  cargo test -p module_05_traits_iterators

use std::fmt;

// ---- Exercise 1: Implement a trait ------------------------------------------
//
// Implement the Summary trait for Article and Tweet.

pub trait Summary {
    fn summarize(&self) -> String;

    // Default implementation — you can override it, but don't have to.
    fn preview(&self) -> String {
        let s = self.summarize();
        if s.len() > 50 {
            format!("{}...", &s[..50])
        } else {
            s
        }
    }
}

pub struct Article {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// Implement Summary for Article.
// summarize() should return: "{title} by {author}"
impl Summary for Article {
    fn summarize(&self) -> String {
        todo!()
    }
}

// Implement Summary for Tweet.
// summarize() should return: "{username}: {content}"
impl Summary for Tweet {
    fn summarize(&self) -> String {
        todo!()
    }
}

// ---- Exercise 2: Display ------------------------------------------------
//
// Implement std::fmt::Display for Matrix so it prints like:
//   [ 1  2 ]
//   [ 3  4 ]

pub struct Matrix {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

// ---- Exercise 3: Generic function -------------------------------------------
//
// Implement a generic function that takes a slice and returns the minimum element.
// Return None if the slice is empty.
pub fn minimum<T: PartialOrd>(list: &[T]) -> Option<&T> {
    todo!()
}

// ---- Exercise 4: Iterator chains --------------------------------------------
//
// Implement these functions using iterator methods (.map, .filter, .sum, etc.)
// No for loops allowed!

// Return the sum of squares of all odd numbers in the slice.
pub fn sum_of_odd_squares(numbers: &[i32]) -> i32 {
    todo!()
}

// Return a Vec of strings, each uppercased and with "!" appended.
// Input: ["hello", "world"] → Output: ["HELLO!", "WORLD!"]
pub fn shout_all(words: &[&str]) -> Vec<String> {
    todo!()
}

// Given a Vec<Vec<i32>>, flatten it into a single Vec<i32> containing only
// positive numbers, sorted in ascending order.
pub fn flatten_and_filter_positive(nested: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

// ---- Exercise 5: Closures ---------------------------------------------------
//
// Return a closure that multiplies its input by `factor`.
pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    todo!()
}

// Apply `f` to each element and return a new Vec.
// This is basically reimplementing .map() — use a for loop here (not .map()).
pub fn apply_to_all<F: Fn(i32) -> i32>(items: &[i32], f: F) -> Vec<i32> {
    todo!()
}

// ---- Tests ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_article() {
        let a = Article {
            title: "Rust is great".to_string(),
            author: "Dylan".to_string(),
            content: "...".to_string(),
        };
        assert_eq!(a.summarize(), "Rust is great by Dylan");
    }

    #[test]
    fn test_summary_tweet() {
        let t = Tweet {
            username: "rustacean".to_string(),
            content: "Hello from Rust!".to_string(),
        };
        assert_eq!(t.summarize(), "rustacean: Hello from Rust!");
    }

    #[test]
    fn test_summary_preview_default() {
        let t = Tweet {
            username: "user".to_string(),
            content: "short".to_string(),
        };
        // Should use default implementation — same as summarize since < 50 chars
        assert_eq!(t.preview(), "user: short");

        let long = Tweet {
            username: "user".to_string(),
            content: "this is a very long tweet that exceeds fifty characters easily".to_string(),
        };
        assert!(long.preview().ends_with("..."));
        assert!(long.preview().len() <= 53); // 50 chars + "..."
    }

    #[test]
    fn test_matrix_display() {
        let m = Matrix { a: 1, b: 2, c: 3, d: 4 };
        let s = format!("{}", m);
        assert!(s.contains("1"));
        assert!(s.contains("2"));
        assert!(s.contains("3"));
        assert!(s.contains("4"));
        // Should have two lines
        assert_eq!(s.lines().count(), 2);
    }

    #[test]
    fn test_minimum() {
        assert_eq!(minimum(&[3, 1, 4, 1, 5, 9]), Some(&1));
        assert_eq!(minimum(&[42]), Some(&42));
        assert_eq!(minimum::<i32>(&[]), None);
        assert_eq!(minimum(&["banana", "apple", "cherry"]), Some(&"apple"));
    }

    #[test]
    fn test_sum_of_odd_squares() {
        // 1² + 3² + 5² = 1 + 9 + 25 = 35
        assert_eq!(sum_of_odd_squares(&[1, 2, 3, 4, 5]), 35);
        assert_eq!(sum_of_odd_squares(&[2, 4, 6]), 0);
        assert_eq!(sum_of_odd_squares(&[]), 0);
    }

    #[test]
    fn test_shout_all() {
        assert_eq!(shout_all(&["hello", "world"]), vec!["HELLO!", "WORLD!"]);
        assert_eq!(shout_all(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_flatten_and_filter_positive() {
        let nested = vec![
            vec![3, -1, 2],
            vec![-5, 4],
            vec![1, -2, 5],
        ];
        assert_eq!(flatten_and_filter_positive(nested), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_make_multiplier() {
        let triple = make_multiplier(3);
        assert_eq!(triple(5), 15);
        assert_eq!(triple(0), 0);
        assert_eq!(triple(-2), -6);
    }

    #[test]
    fn test_apply_to_all() {
        let doubled = apply_to_all(&[1, 2, 3], |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);

        let negated = apply_to_all(&[1, -2, 3], |x| -x);
        assert_eq!(negated, vec![-1, 2, -3]);
    }
}
