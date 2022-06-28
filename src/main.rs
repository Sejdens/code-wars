#![allow(unused)]

mod kata;
use kata::{kyu_6};

fn main() {
    println!("{:?}",
        kyu_6::two_sum::two_sum(&[1, 2, 3], 4)
    );
}
