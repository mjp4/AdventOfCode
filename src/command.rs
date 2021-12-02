#[derive(PartialEq, Debug)]
pub enum CommandMethod {
    Forward,
    Down,
    Up,
}

#[derive(PartialEq, Debug)]
pub struct Command {
    pub method: CommandMethod,
    pub param: isize,
}

impl Command {
    pub fn from_str(command_as_str: &str) -> Command {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
