//! Write a function that takes an array of numbers (integers for the tests) and a target number. It should find two different items in the array that, when added together, give the target value. The indices of these items should then be returned in a tuple / list (depending on your language) like so: (index1, index2).
//!
//! For the purposes of this kata, some tests may have multiple answers; any valid solutions will be accepted.
//!
//! The input will always be valid (numbers will be an array of length 2 or greater, and all of the items will be numbers; target will always be the sum of two different items from that array).
//!
//! Based on: [http://oj.leetcode.com/problems/two-sum/](http://oj.leetcode.com/problems/two-sum/)
//!
//! ```
//! use code_wars::kata::kyu_6::two_sum::two_sum;
//! assert!(two_sum(&[1, 2, 3], 4) == (0, 2) ||
//!     two_sum(&[1, 2, 3], 4) == (2, 0)
//!     )
//! ```
//!
//! Source: [Two Sum](https://www.codewars.com/kata/52c31f8e6605bcc646000082/rust)

pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn sample() {
        do_test(&[1, 2, 3], 4);
        do_test(&[1234, 5678, 9012], 14690);
        do_test(&[2, 2, 3], 4);
    }

    fn do_test(nums: &[i32], sum: i32) {
        let len = nums.len();
        let user_tuple = two_sum(nums, sum);
        assert!(
            user_tuple.0 < len && user_tuple.1 < len,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
            nums,
            sum,
            user_tuple
        );
        assert!(
            user_tuple.0 != user_tuple.1,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices",
            nums, sum, user_tuple
        );
        let num1 = nums[user_tuple.0];
        let num2 = nums[user_tuple.1];
        let user_sum = num1 + num2;
        assert!(
            user_sum == sum,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target",
            nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum
        )
    }
}
