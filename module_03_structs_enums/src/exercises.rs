// Module 03 Exercises — Structs, Enums, Pattern Matching
//
// Implement each item below to make the tests pass.
// Run:  cargo test -p module_03_structs_enums

use std::fmt;

// ---- Exercise 1: Struct with methods ----------------------------------------
//
// Implement the Circle struct and its methods.
// Use std::f64::consts::PI for pi.

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    // Create a new Circle
    pub fn new(radius: f64) -> Circle {
        todo!()
    }

    // Area of the circle
    pub fn area(&self) -> f64 {
        todo!()
    }

    // Circumference of the circle (2 * pi * r)
    pub fn circumference(&self) -> f64 {
        todo!()
    }

    // Return true if this circle is larger (by area) than other
    pub fn is_larger_than(&self, other: &Circle) -> bool {
        todo!()
    }
}

// ---- Exercise 2: Enum with data and methods ---------------------------------
//
// Implement the Temperature enum and its methods.

pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    // Convert to Celsius (if already Celsius, return as-is)
    pub fn to_celsius(&self) -> f64 {
        todo!()
    }

    // Convert to Fahrenheit (if already Fahrenheit, return as-is)
    pub fn to_fahrenheit(&self) -> f64 {
        todo!()
    }

    // Return a human-readable description: e.g. "100°C" or "212°F"
    pub fn describe(&self) -> String {
        todo!()
    }
}

// ---- Exercise 3: Pattern matching -------------------------------------------
//
// Implement a function that classifies a (x, y) coordinate.

pub enum Quadrant {
    First,    // x > 0, y > 0
    Second,   // x < 0, y > 0
    Third,    // x < 0, y < 0
    Fourth,   // x > 0, y < 0
    Origin,   // x == 0, y == 0
    OnXAxis,  // y == 0, x != 0
    OnYAxis,  // x == 0, y != 0
}

impl fmt::Display for Quadrant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Quadrant::First => "Q1",
            Quadrant::Second => "Q2",
            Quadrant::Third => "Q3",
            Quadrant::Fourth => "Q4",
            Quadrant::Origin => "Origin",
            Quadrant::OnXAxis => "X-axis",
            Quadrant::OnYAxis => "Y-axis",
        };
        write!(f, "{}", s)
    }
}

pub fn classify_point(x: i32, y: i32) -> Quadrant {
    todo!()
}

// ---- Exercise 4: Option ---------------------------------------------------------
//
// Implement these functions that use Option.

// Return the first element of a slice, or None if the slice is empty.
pub fn first<T: Clone>(items: &[T]) -> Option<T> {
    todo!()
}

// Divide a by b. Return None if b is zero.
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

// Given a Vec<Option<i32>>, return a Vec<i32> containing only the Some values.
pub fn collect_some(items: Vec<Option<i32>>) -> Vec<i32> {
    todo!()
}

// ---- Tests ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn test_circle_area() {
        let c = Circle::new(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < EPSILON);

        let c2 = Circle::new(5.0);
        assert!((c2.area() - std::f64::consts::PI * 25.0).abs() < EPSILON);
    }

    #[test]
    fn test_circle_circumference() {
        let c = Circle::new(1.0);
        assert!((c.circumference() - 2.0 * std::f64::consts::PI).abs() < EPSILON);
    }

    #[test]
    fn test_circle_is_larger_than() {
        let big = Circle::new(5.0);
        let small = Circle::new(2.0);
        assert!(big.is_larger_than(&small));
        assert!(!small.is_larger_than(&big));
        assert!(!big.is_larger_than(&big));
    }

    #[test]
    fn test_temperature_to_celsius() {
        let t = Temperature::Celsius(100.0);
        assert!((t.to_celsius() - 100.0).abs() < EPSILON);

        let t2 = Temperature::Fahrenheit(212.0);
        assert!((t2.to_celsius() - 100.0).abs() < 0.001);

        let t3 = Temperature::Fahrenheit(32.0);
        assert!((t3.to_celsius() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_temperature_to_fahrenheit() {
        let t = Temperature::Fahrenheit(212.0);
        assert!((t.to_fahrenheit() - 212.0).abs() < EPSILON);

        let t2 = Temperature::Celsius(100.0);
        assert!((t2.to_fahrenheit() - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_temperature_describe() {
        assert_eq!(Temperature::Celsius(100.0).describe(), "100°C");
        assert_eq!(Temperature::Fahrenheit(212.0).describe(), "212°F");
    }

    #[test]
    fn test_classify_point() {
        assert_eq!(classify_point(1, 1).to_string(), "Q1");
        assert_eq!(classify_point(-1, 1).to_string(), "Q2");
        assert_eq!(classify_point(-1, -1).to_string(), "Q3");
        assert_eq!(classify_point(1, -1).to_string(), "Q4");
        assert_eq!(classify_point(0, 0).to_string(), "Origin");
        assert_eq!(classify_point(5, 0).to_string(), "X-axis");
        assert_eq!(classify_point(0, 3).to_string(), "Y-axis");
    }

    #[test]
    fn test_first() {
        assert_eq!(first(&[1, 2, 3]), Some(1));
        assert_eq!(first::<i32>(&[]), None);
        assert_eq!(first(&["a", "b"]), Some("a"));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
        assert_eq!(safe_divide(7.0, 0.0), None);
    }

    #[test]
    fn test_collect_some() {
        let items = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(collect_some(items), vec![1, 3, 5]);

        let all_none: Vec<Option<i32>> = vec![None, None];
        assert_eq!(collect_some(all_none), Vec::<i32>::new());
    }
}
