//! In this kata you have to write a method that folds a given array of integers by the middle x-times.
//!
//! An example says more than thousand words:
//!
//! Fold 1-times:
//!
//! ```
//! # use code_wars::kata::kyu_6::fold_array;
//! assert_eq!(fold_array::run_a(&[1,2,3,4,5], 1), [6,6,3]);
//! ```
//!
//! A little visualization (NOT for the algorithm but for the idea of folding):
//!
//! ```text
//!  Step 1         Step 2        Step 3       Step 4       Step5
//!                      5/           5|         5\
//!                     4/            4|          4\
//! 1 2 3 4 5      1 2 3/         1 2 3|       1 2 3\       6 6 3
//! ----*----      ----*          ----*        ----*        ----*
//! ```
//!
//!
//! Fold 2-times:
//! \[1,2,3,4,5\] -> \[9,6\]
//!
//! ```
//! # use code_wars::kata::kyu_6::fold_array;
//! assert_eq!(fold_array::run_a(&[1,2,3,4,5], 2), [9,6]);
//! ```
//!
//! As you see, if the count of numbers is odd, the middle number will stay. Otherwise the fold-point is between the middle-numbers, so all numbers would be added in a way.
//!
//! The array will always contain numbers and will never be null. The parameter runs will always be a positive integer greater than 0 and says how many runs of folding your method has to do.
//!
//! If an array with one element is folded, it stays as the same array.
//!
//! The input array should not be modified!
//!
//! Have fun coding it and please don't forget to vote and rank this kata! :-)
//!
//! I have created other katas. Have a look if you like coding and challenges.
//!
//! Source: [Fold an array](https://www.codewars.com/kata/57ea70aa5500adfe8a000110/rust)

/// Purely functional
pub fn run_a(arr: &[i32], runs: usize) -> Vec<i32> {
    (0..runs).fold(arr.to_vec(), |mut acc, _| {
        acc.split_off(acc.len() / 2)
            .iter()
            .rev()
            .zip(acc.iter().chain(std::iter::repeat(&0)))
            .map(|(&l, &r)| l + r)
            .collect()
    })
}

/// Slightly faster
pub fn run_b(arr: &[i32], runs: usize) -> Vec<i32> {
    let mut result: Vec<i32> = arr.into();
    let mut half: Vec<i32>;
    for _ in 0..runs {
        half = result.split_off(result.len() / 2);
        result = half
            .iter()
            .rev()
            .zip(result.iter().chain(std::iter::repeat(&0)))
            .map(|(&l, &r)| l + r)
            .collect();
    }
    result
}

/// Fastest
pub fn run_c(arr: &[i32], runs: usize) -> Vec<i32> {
    let mut vec = arr.to_owned();

    for _ in 0..runs {
        let segment_len = vec.len() / 2;
        for i in 0..segment_len {
            vec[i] += vec[vec.len() - i - 1];
        }
        vec.truncate(vec.len() - segment_len)
    }

    vec.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = [1, 2, 3, 4, 5];
        assert_eq!(run_a(&input, 1), [6, 6, 3]);
        assert_eq!(run_a(&input, 2), [9, 6]);
        assert_eq!(run_a(&input, 3), [15]);

        let input = [-9, 9, -8, 8, 66, 23];
        assert_eq!(run_a(&input, 1), [14, 75, 0]);
    }
}
