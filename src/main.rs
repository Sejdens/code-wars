#![allow(unused)]

use code_wars::kata::{kyu_6, kyu_7};
use rand::Rng;
use std::time::Instant;

fn main() {
    // dbg!(kyu_6::unique_in_order::run([1,1,2,2,3,2,2,3,2]));
    
    let rounds = 10;
    let tries = 10_000_000;
    let mut time = 0.;
    let mut rng = rand::thread_rng();
    
    for round in 0..rounds {
        let now = Instant::now();
        let mut value: [i32;20] = rng.gen();
        for _ in 0..tries {
            kyu_6::unique_in_order::run(value);
            value = rng.gen::<[i32;20]>();
        }
        
        time += now.elapsed().as_secs_f64();
    }
    
    println!("Average time: {:.4}s", time / rounds as f64);
}
