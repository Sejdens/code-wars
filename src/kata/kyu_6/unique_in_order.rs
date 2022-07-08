//! Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements with the same value next to each other and preserving the original order of elements.
//! 
//! For example:
//! 
//! ```
//! use code_wars::kata::kyu_6::unique_in_order;
//! assert_eq!(unique_in_order::run_a("AAAABBBCCDAABBB".chars()), vec!['A', 'B', 'C', 'D', 'A', 'B']);
//! assert_eq!(unique_in_order::run_a("ABBCcAD".chars())        , vec!['A', 'B', 'C', 'c', 'A', 'D']);
//! assert_eq!(unique_in_order::run_a([1,2,2,3,3])              , vec![1,2,3]);
//! ```
//! 
//! Source: [Unique In Order](https://www.codewars.com/kata/54e6533c92449cc251001667/rust)

/// Turns sequence into iterator and folds it, pushing into a vector the deduped values.
pub fn run_a<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().fold(Vec::new(), |mut v, i| {
        if let Some(last) = v.last() {
            if last != &i {
                v.push(i);
            }
        } else {
            v.push(i);
        }
        v
    })
}

/// Turns sequence into a vector and uses the dedup method.
pub fn run_b<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut res = sequence.into_iter().collect::<Vec<_>>();
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(run_a("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}
