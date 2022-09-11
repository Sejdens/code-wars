/*! Write a function, which takes a non-negative integer (seconds) as input and returns the time in a human-readable format (HH:MM:SS)
 *
 *  HH = hours, padded to 2 digits, range: 00 - 99
 *  MM = minutes, padded to 2 digits, range: 00 - 59
 *  SS = seconds, padded to 2 digits, range: 00 - 59
 *
 * The maximum time never exceeds 359999 (99:59:59)
 *
 * You can find some examples in the test fixtures.
 * Source: [Human Readable Time](https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust)
 */

pub fn make_readable(seconds: u32) -> String {
    let hours = seconds / 3600;
    let seconds = seconds % 3600;
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::make_readable;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_test(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        do_test(0, "00:00:00");
        do_test(59, "00:00:59");
        do_test(60, "00:01:00");
        do_test(3599, "00:59:59");
        do_test(3600, "01:00:00");
        do_test(86399, "23:59:59");
        do_test(86400, "24:00:00");
        do_test(359999, "99:59:59");
    }
}
