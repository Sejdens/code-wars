//! Complete the solution so that the function will break up camel casing, using a space between words.
//! ## Example
//!
//! ```
//! # use code_wars::kata::kyu_6::break_camel_case;
//! assert_eq!(break_camel_case::run("camelCasing"), "camel Casing");
//! assert_eq!(break_camel_case::run("identifier"), "identifier");
//! assert_eq!(break_camel_case::run(""), "");
//! ```

pub fn run(s: &str) -> String {
    s.chars().fold("".to_string(), |acc, x| {
        if x.is_uppercase() {
            acc + &format!(" {}", x)
        } else {
            acc + &x.to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(run("camelCasing"), "camel Casing");
        assert_eq!(run("camelCasingTest"), "camel Casing Test");
    }
}
