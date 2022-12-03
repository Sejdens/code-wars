//! Take the following IPv4 address: 128.32.10.1
//!
//! This address has 4 octets where each octet is a single byte (or 8 bits).
//!
//! 1st octet 128 has the binary representation: 10000000
//! 2nd octet 32 has the binary representation: 00100000
//! 3rd octet 10 has the binary representation: 00001010
//! 4th octet 1 has the binary representation: 00000001
//!
//! So 128.32.10.1 == 10000000.00100000.00001010.00000001
//!
//! Because the above IP address has 32 bits, we can represent it as the unsigned 32 bit number: 2149583361
//!
//! Complete the function that takes an unsigned 32 bit number and returns a string representation of its IPv4 address.
//!
//! ## Examples
//!
//! ```
//! # use code_wars::kata::kyu_5::int32_to_ip;
//! assert_eq!(int32_to_ip::run(2149583361), "128.32.10.1");
//! assert_eq!(int32_to_ip::run(32), "0.0.0.32");
//! assert_eq!(int32_to_ip::run(0), "0.0.0.0");
//! ```

use std::net::Ipv4Addr;

pub fn run(int: u32) -> String {
    Ipv4Addr::from(int).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(run(2154959208), "128.114.17.104");
        assert_eq!(run(2149583361), "128.32.10.1");
        assert_eq!(run(0), "0.0.0.0");
    }
}
