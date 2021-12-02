use aoc::{run_solution,input_file_path, cargo_input_file_path};
use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let year: usize = matches.value_of_t("year").unwrap_or(2021);
    let day: usize = matches.value_of_t("DAY").unwrap_or(1);
    let puzzle: usize = matches.value_of_t("PUZZLE").unwrap_or(1);
    println!(
        "Running day {}, puzzle {} for Advent of Code {}",
        day, puzzle, year
    );

    let input_path = match matches.value_of("input_dir") {
        Some(dir) => input_file_path(year, day, dir),
        None => cargo_input_file_path(year, day),
    };

    let answer = if input_path.is_file() {
        run_solution(year, day, puzzle, &input_path)
    } else {
        println!("Input file {} does not exist", input_path.display());
        None
    };

    if let Some(a) = answer {
        println!();
        println!("ANSWER: {}", a)
    }
}
