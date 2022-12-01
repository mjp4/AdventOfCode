mod bingo;
mod bitaccumulator;
mod command;
mod coordinates;
mod crabs;
mod lanternfish;
mod position;
mod segment_display;
mod valuemap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use itertools::Itertools;

use crate::bingo::BingoState;
use crate::bitaccumulator::DiagsReport;
use crate::command::Command;
use crate::coordinates::{GridCounter, LineSegment};
use crate::lanternfish::LanternShoal;
use crate::position::Position;
use crate::segment_display::{SegmentDisplay, SegmentMapping};

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
            nonempty_file_lines_as_strings(input_path)
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
        (2021, 4, 1) => Some(
            BingoState::from_strs(5, nonempty_file_lines_as_strings(input_path))
                .run_until(|b| b.any_complete())
                .multiply_complete_sum_unmarked_by_last_number()
                .unwrap(),
        ),
        (2021, 4, 2) => Some(
            BingoState::from_strs(5, nonempty_file_lines_as_strings(input_path))
                .run_until(|b| b.all_complete())
                .multiply_complete_sum_unmarked_by_last_number()
                .unwrap(),
        ),
        (2021, 5, 1) => Some(
            nonempty_file_lines_as_strings(input_path)
                .filter_map(|seg_str| LineSegment::from_str(&seg_str).ok())
                .filter(|ls| ls.is_horiz() || ls.is_vert())
                .flat_map(|ls| ls.coords())
                .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
                .into_values()
                .filter(|&v| v > 1)
                .count(),
        ),
        (2021, 5, 2) => Some(
            nonempty_file_lines_as_strings(input_path)
                .filter_map(|seg_str| LineSegment::from_str(&seg_str).ok())
                .flat_map(|ls| ls.coords())
                .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
                .into_values()
                .filter(|&v| v > 1)
                .count(),
        ),
        (2021, 6, 1) => Some(
            LanternShoal::from_str(&single_line_from_file(input_path))
                .proceed_n_days(80)
                .count(),
        ),
        (2021, 6, 2) => Some(
            LanternShoal::from_str(&single_line_from_file(input_path))
                .proceed_n_days(256)
                .count(),
        ),
        (2021, 7, 1) => {
            println!("Solution no longer available");
            None
        }
        (2021, 7, 2) => {
            let inputs: Vec<usize> = single_line_from_file(input_path)
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let min_cost = (0..).fold_while(usize::MAX, |old_cost, new_position| {
                crabs::fold_step(&inputs, old_cost, new_position)
            });
            Some(min_cost.into_inner())
        }
        (2021, 8, 1) => Some(
            nonempty_file_lines_as_strings(input_path)
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
            nonempty_file_lines_as_strings(input_path)
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
        _ => {
            println!("Puzzle solution not yet available");
            None
        }
    }
}

fn integers_from_file(input_path: &Path) -> impl Iterator<Item = usize> {
    file_lines_as_strings(input_path).filter_map(|s| s.parse::<usize>().ok())
}

fn binary_from_file(input_path: &Path) -> impl Iterator<Item = usize> {
    file_lines_as_strings(input_path).filter_map(|s| usize::from_str_radix(&s, 2).ok())
}

fn nonempty_file_lines_as_strings(input_path: &Path) -> impl Iterator<Item = String> {
    file_lines_as_strings(input_path).filter(|s| !s.is_empty())
}

fn file_lines_as_strings(input_path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(input_path).unwrap();
    BufReader::new(file).lines().filter_map(|l| l.ok())
}

fn single_line_from_file(input_path: &Path) -> String {
    file_lines_as_strings(input_path).next().unwrap()
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
}
