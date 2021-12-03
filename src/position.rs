use std::convert::TryInto;

use crate::command::{Command, CommandMethod};

#[derive(PartialEq, Debug)]
pub struct Position {
    x: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    pub fn new(x: isize, depth: isize, aim: isize) -> Position {
        Position {
            x: x,
            depth: depth,
            aim: aim,
        }
    }

    pub fn at_zero() -> Position {
        Position::new(0, 0, 0)
    }

    pub fn exec_command(&self, command: Command) -> Position {
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

    pub fn multiply_x_by_depth(&self) -> usize {
        println!("{:?}", self);
        (self.x * self.depth).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
