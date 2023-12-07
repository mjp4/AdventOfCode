#![allow(deprecated)]
use aoc::{cargo_input_file_path, file_lines_as_strings, run_solution};
use clap::{load_yaml, App};
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let year: usize = matches.value_of_t("year").unwrap_or(2022);
    let day: usize = matches.value_of_t("DAY").unwrap_or(1);
    let puzzle: usize = matches.value_of_t("PUZZLE").unwrap_or(1);
    println!(
        "Running day {}, puzzle {} for Advent of Code {}",
        day, puzzle, year
    );

    let input_path = match matches.value_of("input_dir") {
        Some(input) => PathBuf::from(input),
        None => cargo_input_file_path(year, day),
    };

    let now = Instant::now();
    let answer = if input_path.is_file() {
        let input_lines = file_lines_as_strings(&input_path);
        run_solution(year, day, puzzle, input_lines)
    } else {
        println!("Input file {} does not exist", input_path.display());
        None
    };
    let duration = now.elapsed().as_micros();

    if let Some(a) = answer {
        println!();
        println!("ANSWER: {} in {} microseconds", a, duration)
    }
}
