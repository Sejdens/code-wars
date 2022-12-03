use std::fs;

pub fn run_1() {
    let file_path = "src/aoc/y22/data/d1.txt";
    let data = fs::read_to_string(file_path);
    if let Ok(calories) = data {
        let calorie_counts = calories.lines().fold(vec![], |mut acc, x| {
            if x.is_empty() {
                acc.push(0);
                acc
            } else {
                *acc.last_mut().unwrap_or(&mut 0) += x.parse::<i32>().unwrap();
                acc
            }
        });

        let largest = calorie_counts.iter().max();
        match largest {
            Some(answer) => println!("The largest calorie count is: {}", answer),
            None => println!("No value found!"),
        }
    } else {
        println!("Error while getting the data!");
    }
}
