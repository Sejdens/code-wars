//! As a part of this Kata, you need to create a function that when provided with a triplet, returns the index of the numerical element that lies between the other two elements.
//! 
//! The input to the function will be an array of three distinct numbers (Haskell: a tuple).
//! 
//! # Example:
//! 
//! ```
//! use code_wars::kata::kyu_7::gimme::gimme;
//! assert_eq!(gimme([2, 3, 1]), 0);
//! ```
//! 
//! 2 is the number that fits between 1 and 3 and the index of 2 in the input array is 0.
//! 
//! Another example (just to make sure it is clear):
//! 
//! ```
//! use code_wars::kata::kyu_7::gimme::gimme;
//! assert_eq!(gimme([5, 10, 14]), 1);
//! ```
//!
//! 10 is the number that fits between 5 and 14 and the index of 10 in the input array is 1.
//! 
//! Source: [Find the middle element](https://www.codewars.com/kata/545a4c5a61aa4c6916000755/train/rust)

pub fn gimme(arr: [i32;3]) -> usize {
    let ab = arr[0] > arr[1];
    if ab ^ (arr[0] > arr[2]) { return 0 }
    if ab ^ (arr[1] > arr[2]) { return 2 }
    1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
