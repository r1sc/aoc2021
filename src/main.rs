use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[macro_use]
mod scan;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

const FUNCTIONS: [for<'r> fn(Vec<&'r str>) -> (i32, i32); 5] = [
    day_1::main,
    day_2::main,
    day_3::main,
    day_4::main,
    day_5::main,
];

fn run_day(day_number: usize) {
    println!("Results for day {}", day_number);

    let data: Vec<_> = BufReader::new(File::open(format!("data-day_{}.txt", day_number)).unwrap()) // Open the file (crash on error)
        .lines() // Get all lines as Result<String>'s
        .flatten() // Remove error lines
        .collect();

    let borrowed: Vec<_> = data.iter().map(String::as_str).collect();

    let now = Instant::now();
    let (part1_answer, part2_answer) = FUNCTIONS[day_number - 1](borrowed);
    let duration = now.elapsed();
    println!(
        "Part 1: {}, part 2: {}. Completed in {} msec ({} µsec)",
        part1_answer,
        part2_answer,
        duration.as_millis(),
        duration.as_micros()
    );
    println!()
}

fn main() {
    run_day(1);
    run_day(2);
    run_day(3);
    run_day(4);
    run_day(5);
}
