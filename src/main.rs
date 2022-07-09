#![allow(unused)]

use code_wars::kata::{kyu_6, kyu_7};
use rand::{Rng, rngs::ThreadRng};
use std::time::Instant;

fn main() {
    use kyu_6::fold_array::run_a;
    
    let rounds = 10;
    let tries = 1_000_000;
    let mut time = 0.;
    let mut rng = rand::thread_rng();
    
    fn gen_rand_arr<const SIZE: usize> (rng: &mut ThreadRng, n: i32) -> [i32; SIZE] {
        let mut arr = [0; SIZE];
        for x in &mut arr {
            *x = rng.gen_range(-n..n);
        }
        arr
    }
    
    let mut a: [i32;20] = gen_rand_arr(&mut rng, 100);
    for _ in 0..rounds {
        let now = Instant::now();

        for _ in 0..tries {
            let b = rng.gen_range(1..5);
            run_a(&a, b);
            a = gen_rand_arr(&mut rng, 100);
        }

        time += now.elapsed().as_secs_f64();
    }
    println!("Average time: {:.4}s", time / rounds as f64);
    run_a(&a, 2);
}
