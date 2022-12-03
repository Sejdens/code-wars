/*! A format for expressing an ordered list of integers is to use a comma separated list of either:
 *
 * - individual integers,
 * - or a range of integers denoted by the starting integer separated from the end integer in the range by a dash, '-'. The range includes all integers in the interval including both endpoints. It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17".
 *
 * Complete the solution so that it takes a list of integers in increasing order and returns a correctly formatted string in the range format.
 *
 * Example:
 *
 * ```
 * # use code_wars::kata::kyu_4::range_extraction;
 * assert_eq!(
 *     range_extraction::run(&[-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]),
 *      "-10--8,-6,-3-1,3-5,7-11,14,15,17-20");
 * ```
 *
 * Courtesy of <rosettacode.org>
 * Source: [Range Extraction](https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/rust)
 */

pub fn run(arr: &[i32]) -> String {
    if arr.is_empty() {
        return "".to_owned();
    }
    let mut ans = vec![arr[0]];
    let mut chained = false;
    let mut tail = 0;
    let mut mods: Vec<(usize, String)> = Vec::new();

    for i in arr.iter().skip(1) {
        if !chained {
            if ans.last().unwrap() + 1 != *i {
                ans.push(*i);
            } else {
                chained = true;
                tail = *i;
            }
        } else if tail + 1 == *i {
            tail = *i;
        } else {
            if ans.last().unwrap() + 1 == tail {
                ans.push(tail);
            } else {
                mods.push((ans.len() - 1, format!("-{}", &tail.to_string())));
            }
            ans.push(*i);
            chained = false;
        }
    }

    if chained {
        if ans.last().unwrap() + 1 == tail {
            ans.push(tail);
        } else {
            mods.push((ans.len() - 1, "-".to_owned() + &tail.to_string()));
        }
    }

    let mut ans = ans.iter().map(|num| num.to_string()).collect::<Vec<_>>();

    for m in &mods {
        ans[m.0] += &m.1
    }
    ans.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            run(&[-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            run(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
        assert_eq!("", run(&[]));
    }
}
