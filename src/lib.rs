mod bingo;
mod bitaccumulator;
mod calories;
mod command;
mod coordinates;
mod crabs;
mod lanternfish;
mod position;
mod rockpaperscissors;
mod segment_display;
mod valuemap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use itertools::Itertools;

use crate::bingo::BingoState;
use crate::bitaccumulator::DiagsReport;
use crate::calories::CalorieCounter;
use crate::command::Command;
use crate::coordinates::{GridCounter, LineSegment};
use crate::lanternfish::LanternShoal;
use crate::position::Position;
use crate::rockpaperscissors::score_guide_round;
use crate::segment_display::{SegmentDisplay, SegmentMapping};

pub fn run_solution(
    year: usize,
    day: usize,
    puzzle: usize,
    input_strings: impl Iterator<Item = String>,
) -> Option<usize> {
    match (year, day, puzzle) {
        (2021, 1, 1) => Some(
            parse_input(input_strings)
                .tuple_windows()
                .filter(|tuple: &(usize, usize)| tuple.1 > tuple.0)
                .count(),
        ),
        (2021, 1, 2) => Some(
            parse_input(input_strings)
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
            nonempty_input_lines(input_strings)
                .map(|command_str| Command::from_str(&command_str))
                .fold(Position::at_zero(), |pos, com| pos.exec_command(com))
                .multiply_x_by_depth(),
        ),
        (2021, 3, 1) => {
            let dr = DiagsReport::from_iter(12, binary_from_input(input_strings));
            Some(dr.gamma_rate() * dr.epsilon_rate())
        }
        (2021, 3, 2) => {
            let dr = DiagsReport::from_iter(12, binary_from_input(input_strings));
            Some(dr.oxygen_rate() * dr.co2_scrub_rate())
        }
        (2021, 4, 1) => Some(
            BingoState::from_strs(5, nonempty_input_lines(input_strings))
                .run_until(|b| b.any_complete())
                .multiply_complete_sum_unmarked_by_last_number()
                .unwrap(),
        ),
        (2021, 4, 2) => Some(
            BingoState::from_strs(5, nonempty_input_lines(input_strings))
                .run_until(|b| b.all_complete())
                .multiply_complete_sum_unmarked_by_last_number()
                .unwrap(),
        ),
        (2021, 5, 1) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|seg_str| LineSegment::from_str(&seg_str).ok())
                .filter(|ls| ls.is_horiz() || ls.is_vert())
                .flat_map(|ls| ls.coords())
                .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
                .into_values()
                .filter(|&v| v > 1)
                .count(),
        ),
        (2021, 5, 2) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|seg_str| LineSegment::from_str(&seg_str).ok())
                .flat_map(|ls| ls.coords())
                .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
                .into_values()
                .filter(|&v| v > 1)
                .count(),
        ),
        (2021, 6, 1) => Some(
            LanternShoal::from_str(&single_line_from_input(input_strings))
                .proceed_n_days(80)
                .count(),
        ),
        (2021, 6, 2) => Some(
            LanternShoal::from_str(&single_line_from_input(input_strings))
                .proceed_n_days(256)
                .count(),
        ),
        (2021, 7, 1) => {
            println!("Solution no longer available");
            None
        }
        (2021, 7, 2) => {
            let inputs: Vec<usize> = single_line_from_input(input_strings)
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let min_cost = (0..).fold_while(usize::MAX, |old_cost, new_position| {
                crabs::fold_step(&inputs, old_cost, new_position)
            });
            Some(min_cost.into_inner())
        }
        (2021, 8, 1) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|display| {
                    display
                        .split(" | ")
                        .map(|segments| segments.split(' ').map(|s| s.to_string()).collect_vec())
                        .collect_tuple::<(_, _)>()
                })
                .fold(0, |outer_count, (input_segs, output_segs)| {
                    let mapping =
                        SegmentMapping::find_valid(&input_segs).expect("No valid mapping");
                    outer_count
                        + output_segs.iter().fold(0, |count, seg| {
                            match SegmentDisplay::from_str_with_mapping(seg, &mapping).to_int() {
                                Some(1) | Some(4) | Some(7) | Some(8) => count + 1,
                                _ => count,
                            }
                        })
                }),
        ),
        (2021, 8, 2) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|display| {
                    display
                        .split(" | ")
                        .map(|segments| segments.split(' ').map(|s| s.to_string()).collect_vec())
                        .collect_tuple::<(_, _)>()
                })
                .fold(0, |outer_sum, (input_segs, output_segs)| {
                    let mapping =
                        SegmentMapping::find_valid(&input_segs).expect("No valid mapping");
                    outer_sum
                        + output_segs
                            .iter()
                            .fold((0, 1000), |(sum, pos), seg| {
                                (
                                    sum + SegmentDisplay::from_str_with_mapping(seg, &mapping)
                                        .to_int()
                                        .unwrap()
                                        * pos,
                                    pos / 10,
                                )
                            })
                            .0
                }),
        ),
        (2022, 1, 1) => Some(
            input_strings
                .fold(CalorieCounter::reset(), CalorieCounter::fold_step)
                .max,
        ),
        (2022, 1, 2) => Some(
            input_strings
                .fold(CalorieCounter::reset(), CalorieCounter::fold_step)
                .top_three_sum(),
        ),
        (2022, 2, 1) => {
            println!("Solution no longer available");
            None
        }
        (2022, 2, 2) => Some(input_strings.map(|s| score_guide_round(&s)).sum()),
        (2022, 3, 1) => None,
        (2022, 3, 2) => None,
        (2022, 4, 1) => None,
        (2022, 4, 2) => None,
        _ => {
            println!("Puzzle solution not yet available");
            None
        }
    }
}

fn parse_input<T: std::str::FromStr, IS>(input_strings: IS) -> impl Iterator<Item = T>
where
    IS: Iterator<Item = String>,
{
    input_strings.filter_map(|s| s.parse::<T>().ok())
}

fn binary_from_input(input_strings: impl Iterator<Item = String>) -> impl Iterator<Item = usize> {
    input_strings.filter_map(|s| usize::from_str_radix(&s, 2).ok())
}

fn nonempty_input_lines(
    input_strings: impl Iterator<Item = String>,
) -> impl Iterator<Item = String> {
    input_strings.filter(|s| !s.is_empty())
}

pub fn file_lines_as_strings(input_path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(input_path).unwrap();
    BufReader::new(file).lines().filter_map(|l| l.ok())
}

fn single_line_from_input(mut input_strings: impl Iterator<Item = String>) -> String {
    input_strings.next().unwrap()
}

pub fn cargo_input_file_path(year: usize, day: usize) -> PathBuf {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("inputs"));
    let filename = format!("input-{:04}-day{:02}", year, day);
    Path::new(&base_dir).join(Path::new(&filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_solution_for_test(year: usize, day: usize, puzzle: usize) -> usize {
        let input_lines = file_lines_as_strings(&cargo_input_file_path(year, day));
        run_solution(year, day, puzzle, input_lines).unwrap()
    }

    fn run_solution_for_example(year: usize, day: usize, puzzle: usize) -> usize {
        let input_lines = example_input(year, day).lines().map(|s| s.to_string());
        run_solution(year, day, puzzle, input_lines).unwrap()
    }

    fn example_input(year: usize, day: usize) -> &'static str {
        match (year, day) {
            (2022, 1) => {
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            }
            (2022, 2) => {
                "A Y
B X
C Z"
            }
            _ => "",
        }
    }

    #[test]
    #[ignore]
    fn check_examples_2022() {
        assert_eq!(run_solution_for_example(2022, 1, 1), 24000);
        assert_eq!(run_solution_for_example(2022, 1, 2), 45000);
        assert_eq!(run_solution_for_example(2022, 2, 2), 12);
    }

    #[test]
    #[ignore]
    fn check_known_solutions_2021() {
        assert_eq!(run_solution_for_test(2021, 1, 1), 1466);
        assert_eq!(run_solution_for_test(2021, 1, 2), 1491);
        assert_eq!(run_solution_for_test(2021, 2, 2), 1947878632);
        assert_eq!(run_solution_for_test(2021, 3, 1), 4006064);
        assert_eq!(run_solution_for_test(2021, 3, 2), 5941884);
        assert_eq!(run_solution_for_test(2021, 4, 1), 2496);
        assert_eq!(run_solution_for_test(2021, 4, 2), 25925);
        assert_eq!(run_solution_for_test(2021, 5, 1), 5084);
        assert_eq!(run_solution_for_test(2021, 5, 2), 17882);
        assert_eq!(run_solution_for_test(2021, 6, 1), 352195);
        assert_eq!(run_solution_for_test(2021, 6, 2), 1600306001288);
        assert_eq!(run_solution_for_test(2021, 7, 2), 92881128);
        assert_eq!(run_solution_for_test(2021, 8, 1), 397);
        assert_eq!(run_solution_for_test(2021, 8, 2), 1027422);
    }

    #[test]
    #[ignore]
    fn check_known_solutions_2022() {
        assert_eq!(run_solution_for_test(2022, 1, 1), 71506);
        assert_eq!(run_solution_for_test(2022, 1, 2), 209603);
    }

    #[test]
    fn check_known_solutions_2023() {}
}
