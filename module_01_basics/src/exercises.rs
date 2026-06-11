// Module 01 Exercises
//
// Implement each function below to make the tests pass.
// Run:  cargo test -p module_01_basics

// Exercise 1
// Convert Celsius to Fahrenheit.
// Formula: F = C * 9/5 + 32
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

// Exercise 2
// Return true if n is even, false otherwise.
pub fn is_even(n: i32) -> bool {
    todo!()
}

// Exercise 3
// Classic FizzBuzz.
// Return "Fizz" for multiples of 3, "Buzz" for multiples of 5,
// "FizzBuzz" for multiples of both, otherwise the number as a string.
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

// Exercise 4
// Count the number of vowels (a, e, i, o, u — lowercase and uppercase) in a string.
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

// Exercise 5
// Return the largest number in a non-empty slice.
// Hint: iterate and keep track of the largest value seen.
pub fn find_max(numbers: &[i32]) -> i32 {
    todo!()
}

// Exercise 6
// Given a Vec<i32>, return a new Vec<i32> containing only the positive numbers.
// Hint: use a for loop and push to a new Vec, or look at .iter().filter().collect()
pub fn keep_positive(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

// ---- Tests ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-9);
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-9);
        assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < 1e-9);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(4));
        assert!(is_even(-2));
        assert!(!is_even(1));
        assert!(!is_even(7));
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(9), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
    }

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("AEIOU"), 5);
        assert_eq!(count_vowels("xyz"), 0);
        assert_eq!(count_vowels("The quick brown fox"), 5);
        assert_eq!(count_vowels(""), 0);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(find_max(&[5, 4, 3, 2, 1]), 5);
        assert_eq!(find_max(&[-1, -5, -2]), -1);
        assert_eq!(find_max(&[42]), 42);
    }

    #[test]
    fn test_keep_positive() {
        assert_eq!(keep_positive(&[1, -2, 3, -4, 5]), vec![1, 3, 5]);
        assert_eq!(keep_positive(&[-1, -2, -3]), Vec::<i32>::new());
        assert_eq!(keep_positive(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(keep_positive(&[0, 1, -1]), vec![1]);
    }
}
