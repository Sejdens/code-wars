use std::fs;

fn get_calorie_counts(calorie_data: String) -> Vec<i32> {
    calorie_data.lines().fold(vec![], |mut acc, x| {
        if x.is_empty() {
            acc.push(0);
            acc
        } else {
            *acc.last_mut().unwrap_or(&mut 0) += x.parse::<i32>().unwrap();
            acc
        }
    })
}
pub fn run_1() {
    let file_path = "src/aoc/y22/data/d1.txt";
    let data = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Error while trying to read file at: {}", file_path));
    let calorie_counts = get_calorie_counts(data);
    let largest = calorie_counts.iter().max();
    match largest {
        Some(answer) => println!("The largest calorie count is: {}", answer),
        None => println!("No value found!"),
    }
}

pub fn run_2() {
    let file_path = "src/aoc/y22/data/d1.txt";
    let data = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Error while trying to read file at: {}", file_path));
    let mut calorie_counts = get_calorie_counts(data);
    calorie_counts.sort();
    let largest = calorie_counts[calorie_counts.len() - 3..=calorie_counts.len() - 1]
        .iter()
        .fold(0, |acc, x| acc + x);
    println!(
        "The sum of the three largest calorie counts is: {}",
        largest
    );
}
