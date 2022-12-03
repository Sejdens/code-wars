//! Write a program that will calculate the number of trailing zeros in a factorial of a given number.
//!
//! N! = 1 * 2 * 3 *  ... * N
//!
//! Be careful 1000! has 2568 digits...
//!
//! For more info, see: <http://mathworld.wolfram.com/Factorial.html>
//! Examples
//!
//! ```
//! # use code_wars::kata::kyu_5::factorial_trailing_zeros;
//! assert_eq!(factorial_trailing_zeros::run_a(6), 1);
//! // 6! = 1 * 2 * 3 * 4 * 5 * 6 = 720 --> 1 trailing zero
//!
//! assert_eq!(factorial_trailing_zeros::run_a(12), 2);
//! // 12! = 479001600 --> 2 trailing zeros
//! ```
//!
//! Hint: You're not meant to calculate the factorial. Find another way to find the number of zeros.
//!
//! Source: [Number of trailing zeros of N!](https://www.codewars.com/kata/52f787eb172a8b4ae1000a34/rust)

// Fastest for bigger numbers
pub fn run_a(n: u64) -> u64 {
    std::iter::successors(Some(5u64), |n| n.checked_mul(5)).fold(0, |acc, x| acc + n / x)
}

// Fastest for smaller numbers
pub fn run_b(n: u64) -> u64 {
    std::iter::successors(Some(n / 5), |&n| Some(n / 5))
        .take_while(|&n| n > 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(run_a(0), 0);
        assert_eq!(run_a(6), 1);
        assert_eq!(run_a(14), 2);
        assert_eq!(run_a(30), 7);
        assert_eq!(run_a(1_000), 249);
        assert_eq!(run_a(100_000), 24_999);
        assert_eq!(run_a(1_000_000_000), 249_999_998);
        assert_eq!(run_a(100_000_000_000), 24_999_999_997);
        assert_eq!(run_a(std::u64::MAX), 4_611_686_018_427_387_890);
    }
}
