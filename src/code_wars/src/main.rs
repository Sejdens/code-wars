pub mod kata;

fn main() {
    let res = kata::kyu_4::sum_intervals::run(&[(-1_000_000_000, 1_000_000_000)]);
    println!("{}", res);
    println!("{}", i32::MAX);
    println!("{}", 1_000_000_000 * 2);
}
