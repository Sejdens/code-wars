//! Write a function that takes a string of braces, and determines if the order of the braces is valid. It should return true if the string is valid, and false if it's invalid.
//!
//! This Kata is similar to the Valid Parentheses Kata, but introduces new characters: brackets [], and curly braces {}. Thanks to @arnedag for the idea!
//!
//! All input strings will be nonempty, and will only consist of parentheses, brackets and curly braces: ()[]{}.
//! What is considered Valid?
//!
//! A string of braces is considered valid if all braces are matched with the correct brace.
//! ## Examples
//!
//! ```
//! # use challenges::code_wars::kata::kyu_6::valid_braces;
//! assert_eq!(valid_braces::run("(){}[]"), true );
//! assert_eq!(valid_braces::run("([{}])"), true );
//! assert_eq!(valid_braces::run("(}"), false);
//! assert_eq!(valid_braces::run("[(])"), false);
//! assert_eq!(valid_braces::run("[({})](]"), false);
//! ```

pub fn run(s: &str) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
        expect_false("(((({{{[");
    }

    fn expect_true(s: &str) {
        assert!(run(s), "Expected {s:?} to be valid. Got false", s = s);
    }

    fn expect_false(s: &str) {
        assert!(!run(s), "Expected {s:?} to be invalid. Got true", s = s);
    }
}
