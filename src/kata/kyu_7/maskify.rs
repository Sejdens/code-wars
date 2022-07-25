//! Usually when you buy something, you're asked whether your credit card number, phone number or answer to your most secret question is still correct. However, since someone could look over your shoulder, you don't want that shown on your screen. Instead, we mask it.

//! Your task is to write a function maskify, which changes all but the last four characters into '#'.
//!
//! ## Examples:
//!
//! ```
//! # use code_wars::kata::kyu_7::maskify::maskify;
//!
//! assert_eq!(maskify("4556364607935616"), "############5616");
//! assert_eq!(maskify("64607935616"), "#######5616");
//! assert_eq!(maskify("1"), "1");
//! assert_eq!(maskify(""), "");
//!
//! // "What was the name of your first pet?"
//!
//! assert_eq!(maskify("Skippy"), "##ippy");
//!
//! assert_eq!(
//!     maskify("Nananananananananananananananana Batman!"),
//!     "####################################man!"
//! );
//! ```
//!
//! Source: [Credit Card Mask](https://www.codewars.com/kata/5412509bd436bd33920011bc/rust)

pub fn maskify(cc: &str) -> String {
    //! Return a String with all characters masked as "#" except the last 4.

    let hide = cc.len().saturating_sub(4);
    "#".repeat(hide) + &cc[hide..]
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
