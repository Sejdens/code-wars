#![allow(unused)]

use code_wars::kata::{kyu_5, kyu_6, kyu_7};
use rand::{rngs::ThreadRng, Rng};
use std::time::Instant;

fn main() {
    use kyu_5::factorial_trailing_zeros::run_a;

    let rounds = 10;
    let tries = 1_000_000;
    let mut time = 0.;
    let mut rng = rand::thread_rng();

    // fn gen_rand_arr<const SIZE: usize> (rng: &mut ThreadRng, n: i32) -> [i32; SIZE] {
    //     let mut arr = [0; SIZE];
    //     for x in &mut arr {
    //         *x = rng.gen_range(-n..n);
    //     }
    //     arr
    // }

    let mut a: u64;
    for _ in 0..rounds {
        let now = Instant::now();

        for _ in 0..tries {
            a = rng.gen();
            // a = rng.gen_range(1..100);
            run_a(a);
        }

        time += now.elapsed().as_secs_f64();
    }
    println!("Average time: {:.4}s", time / (rounds as f64));
}
