// Module 02 Exercises — Ownership and Borrowing
//
// Implement each function to make the tests pass.
// Run:  cargo test -p module_02_ownership
//
// Tip: pay attention to whether parameters use &T, &mut T, or T (owned).
// The tests won't compile if you use the wrong kind.

// Exercise 1
// Return the length of a string without taking ownership of it.
// The caller should still be able to use the string after calling this.
// Hint: the parameter should be a reference.
pub fn string_length(s: &String) -> usize {
    todo!()
}

// Exercise 2
// Append " world" to the string.
// The function must modify the original string (not return a new one).
pub fn append_world(s: &mut String) {
    todo!()
}

// Exercise 3
// Return the first word in a string (everything before the first space).
// If there is no space, return the whole string.
// The return type is &str — a slice of the input. Don't allocate a new String.
pub fn first_word(s: &str) -> &str {
    todo!()
}

// Exercise 4
// Return the sum of all elements in a slice.
// The Vec should still be usable after calling this function.
pub fn sum_slice(numbers: &[i32]) -> i32 {
    todo!()
}

// Exercise 5
// Return the longer of the two string slices.
// Both strings should still be usable after calling this.
// Hint: you need a lifetime annotation — look at the README section on lifetimes.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

// Exercise 6
// Given a Vec<String>, return a new Vec<String> with each string uppercased.
// Clone the strings — don't consume the input Vec.
pub fn uppercase_all(words: &[String]) -> Vec<String> {
    todo!()
}

// ---- Tests ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_length() {
        let s = String::from("hello");
        assert_eq!(string_length(&s), 5);
        assert_eq!(s, "hello"); // s must still be valid
    }

    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word("one two three"), "one");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_sum_slice() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_slice(&v), 15);
        assert_eq!(v.len(), 5); // v must still be valid
        assert_eq!(sum_slice(&[]), 0);
        assert_eq!(sum_slice(&[-1, 1]), 0);
    }

    #[test]
    fn test_longer() {
        assert_eq!(longer("short", "much longer"), "much longer");
        assert_eq!(longer("equal", "equal"), "equal");
        assert_eq!(longer("longer string", "short"), "longer string");
    }

    #[test]
    fn test_uppercase_all() {
        let words = vec![
            String::from("hello"),
            String::from("world"),
        ];
        let result = uppercase_all(&words);
        assert_eq!(result, vec!["HELLO", "WORLD"]);
        assert_eq!(words.len(), 2); // original must still be valid
    }
}
