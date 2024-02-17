/*! Write a function called sumIntervals/sum_intervals that accepts an array of intervals, and returns the sum of all the interval lengths. Overlapping intervals should only be counted once.
 * Intervals
 *
 * Intervals are represented by a pair of integers in the form of an array. The first value of the interval will always be less than the second value. Interval example: [1, 5] is an interval from 1 to 5. The length of this interval is 4.
 * Overlapping Intervals
 *
 * List containing overlapping intervals:
 *
 * ```
 * [
 *    [1, 4],
 *    [7, 10],
 *    [3, 5]
 * ]
 * ```
 *
 * The sum of the lengths of these intervals is 7. Since [1, 4] and [3, 5] overlap, we can treat the interval as [1, 5], which has a length of 4.
 * Examples:
 *
 * ```
 * # use challenges::code_wars::kata::kyu_4::sum_intervals;
 * assert_eq!(
 *     sum_intervals::run([[1, 2], [6, 10], [11, 15]]),
 *     9
 * );
 *
 *
 * assert_eq!(
 *     sum_intervals::run([[1, 4], [7, 10], [3, 5]],
 *     7
 * );
 *
 * assert_eq!(
 *     sum_intervals::run([[1, 5], [10, 20], [1, 6], [16, 19], [5, 11]],
 *     19
 * );
 *
 * assert_eq!(
 *     s_imIntervals::run([[0, 20], [-100000000, 10], [30, 40]],
 *     100000030
 * )
 * ```
 *
 * Tests with large intervals
 *
 * Your algorithm should be able to handle large intervals. All tested intervals are subsets of the range [-1000000000, 1000000000].
 */

pub fn run(intervals: &[(i32, i32)]) -> i32 {
    let mut vec = intervals.to_vec();
    vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut total = 0;
    let mut curr_max = i32::MIN;
    for (a, b) in vec {
        let bottom = if curr_max > a { curr_max } else { a };
        if bottom < b {
            total += b - bottom;
            curr_max = b;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(run(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(run(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(run(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(run(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            run(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            run(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }
}
