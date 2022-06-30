#![allow(unused)]

use code_wars::kata::{kyu_6, kyu_7};
use rand;
use std::time::Instant;

fn main() {
    let rounds = 100;
    let tries = 10_000_000;
    let mut time = 0.0;
    
    for round in 0..rounds {
        let now = Instant::now();
        let mut value = rand::random::<[i32;3]>();
        for _ in 0..tries {
            kyu_7::gimme::gimme(value);
            value = rand::random();
        }
        
        time += now.elapsed().as_secs_f64();
    }
    
    println!("Average time: {:.4}s", time / rounds as f64);
}
