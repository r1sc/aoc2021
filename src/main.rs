use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[macro_use]
mod scan;
mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_15;
mod day_16;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn run_day<A, B>(day_number: usize, f: impl FnOnce(Vec<&str>) -> (A, B))
where
    A: Display,
    B: Display,
{
    println!("Results for day {}", day_number);

    let data: Vec<_> = BufReader::new(File::open(format!("data-day_{}.txt", day_number)).unwrap()) // Open the file (crash on error)
        .lines() // Get all lines as Result<String>'s
        .flatten() // Remove error lines
        .collect();

    let borrowed: Vec<_> = data.iter().map(String::as_str).collect();

    let now = Instant::now();
    let (part1_answer, part2_answer) = f(borrowed);
    let duration = now.elapsed();
    println!(
        "Part 1: {}, part 2: {}. Completed in {} msec ({} µsec)",
        part1_answer,
        part2_answer,
        duration.as_millis(),
        duration.as_micros()
    );
    println!();
}

fn main() {
    run_day(1, day_1::main);
    run_day(2, day_2::main);
    run_day(3, day_3::main);
    run_day(4, day_4::main);
    run_day(5, day_5::main);
    run_day(6, day_6::main);
    //run_day(7, day_7::main);
    run_day(8, day_8::main);
    run_day(9, day_9::main);
    run_day(10, day_10::main);
    run_day(11, day_11::main);
    // run_day(12, day_12::main);
    run_day(15, day_15::main);
    run_day(16, day_16::main);
}
