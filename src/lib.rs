use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::convert::TryInto;

use itertools::Itertools;

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
        _ => {
            println!("Puzzle solution not yet available");
            None
        }
    }
}

#[derive(PartialEq, Debug)]
enum CommandMethod {
    Forward,
    Down,
    Up,
}

#[derive(PartialEq, Debug)]
struct Command {
    method: CommandMethod,
    param: isize,
}

impl Command {
    fn from_str(command_as_str: &str) -> Command {
        let split: Vec<&str> = command_as_str.split(" ").collect();
        match split[0] {
            "forward" => Command {
                method: CommandMethod::Forward,
                param: split[1].parse::<isize>().unwrap(),
            },
            "down" => Command {
                method: CommandMethod::Down,
                param: split[1].parse::<isize>().unwrap(),
            },
            "up" => Command {
                method: CommandMethod::Up,
                param: split[1].parse::<isize>().unwrap(),
            },
            _ => panic!("Unknown command"),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Position {
    x: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    fn new(x: isize, depth: isize, aim: isize) -> Position {
        Position {
            x: x,
            depth: depth,
            aim: aim,
        }
    }

    fn at_zero() -> Position {
        Position {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn exec_command(&self, command: Command) -> Position {
        match command.method {
            CommandMethod::Forward => self
                .increase_x(command.param)
                .increase_depth(self.aim * command.param),
            CommandMethod::Down => self.increase_aim(command.param),
            CommandMethod::Up => self.increase_aim(-command.param),
        }
    }

    fn increase_x(&self, amount: isize) -> Position {
        Position::new(self.x + amount, self.depth, self.aim)
    }

    fn increase_depth(&self, amount: isize) -> Position {
        Position::new(self.x, self.depth + amount, self.aim)
    }

    fn increase_aim(&self, amount: isize) -> Position {
        Position::new(self.x, self.depth, self.aim + amount)
    }

    fn multiply_x_by_depth(&self) -> usize {
        println!("{:?}", self);
        (self.x * self.depth).try_into().unwrap()
    }
}

fn integers_from_file(input_path: &Path) -> impl Iterator<Item = usize> {
    strings_from_file(input_path)
        .map(|s| s.parse::<usize>())
        .flatten()
}

fn strings_from_file(input_path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    reader
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
    }

    #[test]
    fn check_commands_from_str() {
        assert_eq!(
            Command::from_str("forward 5"),
            Command {
                method: CommandMethod::Forward,
                param: 5,
            }
        );
        assert_eq!(
            Command::from_str("down 8"),
            Command {
                method: CommandMethod::Down,
                param: 8,
            }
        );
        assert_eq!(
            Command::from_str("up 3"),
            Command {
                method: CommandMethod::Up,
                param: 3,
            }
        );
    }

    #[test]
    fn check_postion_change() {
        assert_eq!(
            Position::at_zero()
                .exec_command(Command::from_str("forward 5"))
                .exec_command(Command::from_str("down 5"))
                .exec_command(Command::from_str("forward 8"))
                .exec_command(Command::from_str("up 3"))
                .exec_command(Command::from_str("down 8"))
                .exec_command(Command::from_str("forward 2"))
                .multiply_x_by_depth(),
            900
        )
    }
}
