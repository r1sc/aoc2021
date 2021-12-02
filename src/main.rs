use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod day_1;
mod day_2;

const FUNCTIONS: [fn(Vec<String>); 2] = [day_1::main, day_2::main];

fn run_day(day_number: usize) {
    println!("Results for day {}", day_number);

    let data: Vec<String> =
        BufReader::new(File::open(format!("data-day_{}.txt", day_number)).unwrap()) // Open the file (crash on error)
            .lines() // Get all lines as Result<String>'s
            .flatten() // Remove error lines
            .collect();

    FUNCTIONS[day_number - 1](data)
}

fn main() {
    run_day(2)
}
