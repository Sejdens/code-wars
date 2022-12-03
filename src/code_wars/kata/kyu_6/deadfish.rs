//! Write a simple parser that will parse and run Deadfish.
//!
//! Deadfish has 4 commands, each 1 character long:
//!
//! - i increments the value (initially 0)
//! - d decrements the value
//! - s squares the value
//! - o outputs the value into the return array
//!
//! Invalid characters should be ignored.
//!
//! ```
//! # use code_wars::kata::kyu_6::deadfish;
//! assert_eq!(deadfish::run("iiisdoso"), [ 8, 64 ]);
//! ```

pub fn run(code: &str) -> Vec<i32> {
    let mut res = Vec::new();
    let mut val: i32 = 0;

    for c in code.chars() {
        match c {
            'i' => val += 1,
            'd' => val -= 1,
            's' => val = val.pow(2),
            'o' => res.push(val),
            _ => (),
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(run("iiisdoso"), vec![8, 64]);
        assert_eq!(run("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}
