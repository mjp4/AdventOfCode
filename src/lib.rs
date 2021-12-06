mod bingo;
mod bitaccumulator;
mod command;
mod position;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use crate::bingo::BingoState;
use crate::bitaccumulator::DiagsReport;
use crate::command::Command;
use crate::position::Position;

pub fn run_solution(year: usize, day: usize, puzzle: usize, input_path: &Path) -> Option<usize> {
    match (year, day, puzzle) {
        (2021, 1, 1) => Some(
            integers_from_file(input_path)
                .tuple_windows()
                .filter(|tuple: &(usize, usize)| tuple.1 > tuple.0)
                .count(),
        ),
        (2021, 1, 2) => Some(
            integers_from_file(input_path)
                .tuple_windows()
                .map(|tuple: (usize, usize, usize)| tuple.0 + tuple.1 + tuple.2)
                .tuple_windows()
                .filter(|tuple: &(usize, usize)| tuple.1 > tuple.0)
                .count(),
        ),
        (2021, 2, 1) => {
            println!("Solution no longer available");
            None
        }
        (2021, 2, 2) => Some(
            strings_from_file(input_path)
                .map(|command_str| Command::from_str(&command_str))
                .fold(Position::at_zero(), |pos, com| pos.exec_command(com))
                .multiply_x_by_depth(),
        ),
        (2021, 3, 1) => {
            let dr = DiagsReport::from_iter(12, binary_from_file(input_path));
            Some(dr.gamma_rate() * dr.epsilon_rate())
        }
        (2021, 3, 2) => {
            let dr = DiagsReport::from_iter(12, binary_from_file(input_path));
            Some(dr.oxygen_rate() * dr.co2_scrub_rate())
        }
        (2021, 4, 1) => {
            let mut input_lines = strings_from_file(input_path);
            let first_line = input_lines.next().unwrap();
            let mut input_numbers = first_line.split(",").map(|s| s.parse::<usize>()).flatten();
            let state = BingoState::from_strs(5, input_lines);

            input_numbers
                .fold_while(state, |bs, next_number| {
                    let next_state = bs.handle_number(next_number);
                    if next_state.any_complete() {
                        Done(next_state)
                    } else {
                        Continue(next_state)
                    }
                })
                .into_inner()
                .multiply_complete_sum_unmarked_by_last_number()
        }
        (2021, 4, 2) => {
            let mut input_lines = strings_from_file(input_path);
            let first_line = input_lines.next().unwrap();
            let mut input_numbers = first_line.split(",").map(|s| s.parse::<usize>()).flatten();
            let state = BingoState::from_strs(5, input_lines);

            input_numbers
                .fold_while(state, |bs, next_number| {
                    let next_state = bs.handle_number(next_number);
                    if next_state.all_complete() {
                        Done(next_state)
                    } else {
                        Continue(next_state)
                    }
                })
                .into_inner()
                .multiply_complete_sum_unmarked_by_last_number()
        }
        _ => {
            println!("Puzzle solution not yet available");
            None
        }
    }
}

fn integers_from_file(input_path: &Path) -> impl Iterator<Item = usize> {
    strings_from_file(input_path)
        .map(|s| s.parse::<usize>())
        .flatten()
}

fn binary_from_file(input_path: &Path) -> impl Iterator<Item = usize> {
    strings_from_file(input_path)
        .map(|s| usize::from_str_radix(&s, 2))
        .flatten()
}

fn strings_from_file(input_path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(input_path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line_result| line_result.unwrap())
        .filter(|s| !s.is_empty())
}

pub fn input_file_path(year: usize, day: usize, base_dir: &str) -> PathBuf {
    let filename = format!("input-{:04}-day{:02}", year, day);
    Path::new(base_dir).join(Path::new(&filename))
}

pub fn cargo_input_file_path(year: usize, day: usize) -> PathBuf {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("inputs"));
    input_file_path(year, day, base_dir.to_str().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_solution_for_test(year: usize, day: usize, puzzle: usize) -> usize {
        run_solution(year, day, puzzle, &cargo_input_file_path(year, day)).unwrap()
    }

    #[test]
    fn check_known_solutions() {
        assert_eq!(run_solution_for_test(2021, 1, 1), 1466);
        assert_eq!(run_solution_for_test(2021, 1, 2), 1491);
        assert_eq!(run_solution_for_test(2021, 2, 2), 1947878632);
        assert_eq!(run_solution_for_test(2021, 3, 1), 4006064);
    }
}
